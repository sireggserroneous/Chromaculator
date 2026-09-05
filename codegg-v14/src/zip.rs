//! zip.rs -- WS-Z, THE ZIP PEEL (v14-N4).
//!
//! A ZIP is not one spelling of one value; it is MANY, side by side, wrapped in
//! bookkeeping that must come back byte for byte. So this peel is the deflate
//! peel run per member, plus an exact account of everything between the members.
//!
//! Measured before it was written, on two real archives and their rivals:
//!
//! ```text
//!   python312.zip  599 deflate members  ours 3,753,980  precomp+zpaq 1,847,299  -50.79%
//!   ipf-alienware  31 deflate + 3 store ours 5,653,821  precomp+zpaq 2,534,113  -55.18%
//! ```
//!
//! zpaq alone beats us by 0.2-0.3% on those, because an unpeeled ZIP is
//! incompressible to everyone -- **the whole gap is the peel.** Our own coder on
//! the concatenated inflated members lands at 1,865,974, within 1.0% of the
//! rival's COMPLETE number while still owing its recipe.
//!
//! THE SHAPE: a file is a run of alternating spans -- gap, member, gap, member,
//! ..., gap -- with `gaps.len() == members.len() + 1`. A gap is carried
//! VERBATIM and is never modelled as anything but bytes: local headers, the
//! central directory, the EOCD, and every member this peel did not take. That
//! last clause is the important one. A member is taken only if `deflate::peel`
//! reads it AND re-spells it exactly; a stored member, a method we do not know,
//! or a body that refuses simply stays in the gap and the archive is still
//! peeled around it.
//!
//! `intellij.libraries.icu4j.jar` is why: 35.6 MB, 5,826 members, **all
//! stored, zero deflate**. Nothing to peel. A ZIP peel that assumed deflate
//! would refuse the file or, worse, expand it; this one takes no members, finds
//! nothing to gain, and is passed over by the argmin like any other losing arm.
//!
//! The member offsets come from `peel::members`, which reads the central
//! directory and then computes each body from its own LOCAL header -- the two
//! disagree on name and extra lengths, and using the wrong one silently reads
//! the wrong bytes.

use crate::deflate;

/// the container, peeled: verbatim spans and the members between them
pub struct Zip {
    /// n+1 spans carried byte for byte, in file order
    pub gaps: Vec<Vec<u8>>,
    /// n peeled members, in file order
    pub members: Vec<deflate::Deflate>,
    /// how many inflated bytes each member owns, so `expand` can split them
    pub vlens: Vec<usize>,
    /// every member's inflated bytes, concatenated
    pub values: Vec<u8>,
}

impl Zip {
    pub fn describe(&self) -> String {
        let gapb: usize = self.gaps.iter().map(|g| g.len()).sum();
        let vals: usize = self.vlens.iter().sum();
        let rec: usize = self.members.iter().map(deflate::blob_len).sum();
        let pred = self.members.iter().filter(|m| m.pred.is_some()).count();
        format!(
            "zip: {} members peeled, {} B inflated; recipe {} B verbatim frame + {} B of member recipes ({} predicted, {} stored)",
            self.members.len(),
            vals,
            gapb,
            rec,
            pred,
            self.members.len() - pred
        )
    }
}

/// does this file's own layout offer a ZIP with something to peel? Reads the
/// central directory -- the container's arithmetic -- and never guesses.
pub fn nominates(src: &[u8]) -> bool {
    match crate::peel::members(src) {
        Some(ms) => ms.iter().any(|m| m.method == 8 && m.len > 0),
        None => false,
    }
}

/// peel every deflate member the parse can read AND re-spell, and carry
/// everything else verbatim.
pub fn peel(src: &[u8]) -> Result<Zip, String> {
    let mut ms = crate::peel::members(src).ok_or("not a ZIP layout this build can read")?;
    ms.sort_by_key(|m| m.off);
    // the spans must not overlap: a central directory that says they do is one
    // this peel refuses rather than reassembles into the wrong bytes
    for w in ms.windows(2) {
        if w[0].off + w[0].len > w[1].off {
            return Err(format!("ZIP members overlap at {} and {}", w[0].off, w[1].off));
        }
    }
    // The per-member peel is the row's clock: 599 members x up to 81 lockstep
    // passes each is 33.7 s of a 71.5 s row. Members are independent and each
    // inference is deterministic, so this is byte-identical -- the results are
    // sorted back into file order before a single gap is cut.
    //
    // A WORK-STEALING cursor rather than fixed chunks, because the members are
    // wildly uneven: one member of the 599 takes 4,047 ms against a 100.7 s
    // total, so a chunked split would leave one thread holding it while the
    // rest idled. N2b's law applies here unchanged -- the pool finishes when
    // its SLOWEST task finishes, which bounds this at ~8.3x however many cores
    // are thrown at it.
    let cand: Vec<(usize, &crate::peel::Member)> = ms
        .iter()
        .enumerate()
        .filter(|(_, m)| m.method == 8 && m.len > 0 && m.off + m.len <= src.len())
        .collect();
    let next = std::sync::atomic::AtomicUsize::new(0);
    let lanes = std::thread::available_parallelism().map(|n| n.get()).unwrap_or(4).min(cand.len().max(1));
    let taken: Vec<(usize, deflate::Deflate)> = std::thread::scope(|sc| {
        let hs: Vec<_> = (0..lanes)
            .map(|_| {
                let cand = &cand;
                let next = &next;
                sc.spawn(move || {
                    let mut mine: Vec<(usize, deflate::Deflate)> = Vec::new();
                    loop {
                        let k = next.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                        let Some(&(idx, m)) = cand.get(k) else { break };
                        let body = &src[m.off..m.off + m.len];
                        let Ok(d) = deflate::peel(body) else { continue };
                        // THE LAW, per member and before it is taken: a member
                        // that does not re-spell to its own bytes stays in the
                        // gap, and the archive is still peeled around it.
                        match deflate::respell(&d) {
                            Ok(back) if back == body => {}
                            _ => continue,
                        }
                        mine.push((idx, d));
                    }
                    mine
                })
            })
            .collect();
        hs.into_iter().flat_map(|h| h.join().expect("a ZIP member lane")).collect()
    });
    let mut taken = taken;
    taken.sort_by_key(|(i, _)| *i);

    let mut z = Zip { gaps: Vec::new(), members: Vec::new(), vlens: Vec::new(), values: Vec::new() };
    let mut cur = 0usize;
    for (idx, d) in taken {
        let m = &ms[idx];
        if m.off < cur {
            return Err("a ZIP member starts before the previous one ended".into());
        }
        z.gaps.push(src[cur..m.off].to_vec());
        z.vlens.push(d.values.len());
        z.members.push(d);
        cur = m.off + m.len;
    }
    if z.members.is_empty() {
        return Err("no ZIP member could be peeled".into());
    }
    z.gaps.push(src[cur..].to_vec());
    Ok(z)
}

/// the values as ONE stream, taken out of the members. Called only after THE
/// LAW's re-spell has run, exactly like the deflate peel's own `take_values`:
/// until then each member needs its own inflated bytes to re-spell, and a
/// second concatenated copy would double a large archive in memory.
pub fn take_values(z: &mut Zip) -> Vec<u8> {
    let mut out = Vec::with_capacity(z.vlens.iter().sum());
    for d in z.members.iter_mut() {
        out.append(&mut d.values);
    }
    out
}

/// gaps and members back into the original bytes
pub fn respell(z: &Zip) -> Result<Vec<u8>, String> {
    if z.gaps.len() != z.members.len() + 1 {
        return Err(format!("a ZIP peel with {} gaps around {} members", z.gaps.len(), z.members.len()));
    }
    let mut out = Vec::with_capacity(z.gaps.iter().map(|g| g.len()).sum::<usize>() + z.vlens.iter().sum::<usize>() / 3);
    for (i, d) in z.members.iter().enumerate() {
        out.extend_from_slice(&z.gaps[i]);
        out.extend_from_slice(&deflate::respell(d)?);
    }
    out.extend_from_slice(&z.gaps[z.members.len()]);
    Ok(out)
}

/// hand each member back its own slice of the values, then rebuild any
/// predicted parse. The decode side's counterpart to `peel`.
pub fn expand(z: &mut Zip) -> Result<(), String> {
    if z.vlens.len() != z.members.len() {
        return Err("a ZIP peel whose value lengths do not match its members".into());
    }
    let want: usize = z.vlens.iter().sum();
    if want != z.values.len() {
        return Err(format!("the ZIP recipe expects {} inflated bytes, the values restored {}", want, z.values.len()));
    }
    let mut at = 0usize;
    let values = std::mem::take(&mut z.values);
    for (d, &n) in z.members.iter_mut().zip(z.vlens.iter()) {
        d.values = values[at..at + n].to_vec();
        at += n;
        deflate::expand(d)?;
    }
    Ok(())
}

/// the header: version, member count, and the inflated total
const HDR: usize = 13;

pub fn blob(z: &Zip) -> Vec<u8> {
    let mut b = Vec::new();
    b.push(1u8); // version
    b.extend_from_slice(&(z.members.len() as u32).to_le_bytes());
    b.extend_from_slice(&(z.vlens.iter().sum::<usize>() as u64).to_le_bytes());
    debug_assert_eq!(b.len(), HDR);
    for g in &z.gaps {
        b.extend_from_slice(&(g.len() as u32).to_le_bytes());
    }
    let blobs: Vec<Vec<u8>> = z.members.iter().map(deflate::blob).collect();
    for m in &blobs {
        b.extend_from_slice(&(m.len() as u32).to_le_bytes());
    }
    for g in &z.gaps {
        b.extend_from_slice(g);
    }
    for m in &blobs {
        b.extend_from_slice(m);
    }
    b
}

pub fn from_blob(b: &[u8]) -> Result<Zip, String> {
    if b.len() < HDR || b[0] != 1 {
        return Err("a ZIP recipe this build cannot read".into());
    }
    let n = u32::from_le_bytes([b[1], b[2], b[3], b[4]]) as usize;
    let values_len = u64::from_le_bytes(b[5..13].try_into().unwrap()) as usize;
    // every table and span is bounded against the blob's own length before a
    // byte of it is believed, exactly as the deflate layout does it
    let ngap = n.checked_add(1).ok_or("a ZIP recipe whose member count overflows")?;
    let tab = ngap.checked_add(n).and_then(|k| k.checked_mul(4)).ok_or("a ZIP recipe whose tables overflow")?;
    if HDR + tab > b.len() {
        return Err("a ZIP recipe whose tables run past its end".into());
    }
    let u32at = |i: usize| u32::from_le_bytes([b[i], b[i + 1], b[i + 2], b[i + 3]]) as usize;
    let glens: Vec<usize> = (0..ngap).map(|i| u32at(HDR + i * 4)).collect();
    let mlens: Vec<usize> = (0..n).map(|i| u32at(HDR + ngap * 4 + i * 4)).collect();
    let mut p = HDR + tab;
    let mut sect = |k: usize| -> Result<(usize, usize), String> {
        let e = p.checked_add(k).ok_or("a ZIP recipe section overflows")?;
        if e > b.len() {
            return Err(format!("a ZIP recipe section of {} B runs past its {} B", k, b.len()));
        }
        let r = (p, e);
        p = e;
        Ok(r)
    };
    let mut gaps = Vec::with_capacity(ngap);
    for &k in &glens {
        let (s, e) = sect(k)?;
        gaps.push(b[s..e].to_vec());
    }
    let mut members = Vec::with_capacity(n);
    let mut vlens = Vec::with_capacity(n);
    for &k in &mlens {
        let (s, e) = sect(k)?;
        vlens.push(deflate::layout(&b[s..e])?.values_len as usize);
        members.push(deflate::from_blob(&b[s..e])?);
    }
    if p != b.len() {
        return Err(format!("a ZIP recipe of {} B whose sections account for {}", b.len(), p));
    }
    if vlens.iter().sum::<usize>() != values_len {
        return Err("a ZIP recipe whose members disagree with its inflated total".into());
    }
    Ok(Zip { gaps, members, vlens, values: Vec::new() })
}

/// the inflated total a recipe expects, without parsing the whole thing
pub fn values_len(b: &[u8]) -> Result<usize, String> {
    if b.len() < HDR || b[0] != 1 {
        return Err("a ZIP recipe this build cannot read".into());
    }
    Ok(u64::from_le_bytes(b[5..13].try_into().unwrap()) as usize)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// a ZIP built here rather than found on disk, so the test says what it
    /// tests: two deflate members with a stored one between them, and the
    /// local headers deliberately carrying different name/extra lengths from
    /// the central directory -- the disagreement that makes reading the wrong
    /// header silently read the wrong bytes.
    fn build_zip(bodies: &[(&[u8], u16)]) -> Vec<u8> {
        let mut z: Vec<u8> = Vec::new();
        let mut cd: Vec<(usize, u16, usize, usize)> = Vec::new(); // lho, method, csize, idx
        for (i, (body, method)) in bodies.iter().enumerate() {
            let lho = z.len();
            let name = format!("m{}", i);
            z.extend_from_slice(b"PK\x03\x04");
            z.extend_from_slice(&[20, 0, 0, 0]);
            z.extend_from_slice(&method.to_le_bytes());
            z.extend_from_slice(&[0u8; 16]); // time, date, crc, csize, usize -> name len lands at 26
            z.extend_from_slice(&(name.len() as u16).to_le_bytes());
            z.extend_from_slice(&4u16.to_le_bytes()); // LOCAL extra len 4
            z.extend_from_slice(name.as_bytes());
            z.extend_from_slice(b"XTRA");
            z.extend_from_slice(body);
            cd.push((lho, *method, body.len(), i));
        }
        let cdoff = z.len();
        for (lho, method, csize, i) in &cd {
            let name = format!("m{}", i);
            z.extend_from_slice(b"PK\x01\x02");
            z.extend_from_slice(&[20, 0, 20, 0, 0, 0]);
            z.extend_from_slice(&method.to_le_bytes());
            z.extend_from_slice(&[0u8; 8]); // time, date, crc
            z.extend_from_slice(&(*csize as u32).to_le_bytes());
            z.extend_from_slice(&(*csize as u32).to_le_bytes());
            z.extend_from_slice(&(name.len() as u16).to_le_bytes());
            z.extend_from_slice(&0u16.to_le_bytes()); // CENTRAL extra len 0 -- NOT the local 4
            z.extend_from_slice(&[0u8; 10]); // comment, disk, internal, external -> lho lands at 42
            z.extend_from_slice(&(*lho as u32).to_le_bytes());
            z.extend_from_slice(name.as_bytes());
        }
        let cdsize = z.len() - cdoff;
        z.extend_from_slice(b"PK\x05\x06");
        z.extend_from_slice(&[0, 0, 0, 0]);
        z.extend_from_slice(&(cd.len() as u16).to_le_bytes());
        z.extend_from_slice(&(cd.len() as u16).to_le_bytes());
        z.extend_from_slice(&(cdsize as u32).to_le_bytes());
        z.extend_from_slice(&(cdoff as u32).to_le_bytes());
        z.extend_from_slice(&[0, 0]);
        z
    }

    /// a raw deflate stream of `data`, made by this build's own writer so the
    /// test does not depend on an external tool
    fn raw_deflate(data: &[u8]) -> Vec<u8> {
        // one stored block per 65,535 B, final flag on the last
        let mut out = Vec::new();
        let mut i = 0usize;
        loop {
            let n = (data.len() - i).min(65_535);
            out.push(u8::from(i + n >= data.len()));
            out.extend_from_slice(&(n as u16).to_le_bytes());
            out.extend_from_slice(&(!(n as u16)).to_le_bytes());
            out.extend_from_slice(&data[i..i + n]);
            i += n;
            if i >= data.len() {
                break;
            }
        }
        out
    }

    #[test]
    fn peels_the_deflate_members_and_re_spells_the_archive() {
        let a = raw_deflate(b"the same words the same words the same words");
        let b = raw_deflate(&b"payload".repeat(400));
        let stored: Vec<u8> = b"not compressed at all".to_vec();
        let src = build_zip(&[(&a, 8), (&stored, 0), (&b, 8)]);
        let z = peel(&src).expect("a ZIP with deflate members peels");
        assert_eq!(z.members.len(), 2, "the stored member must not be taken");
        assert_eq!(z.gaps.len(), 3);
        assert_eq!(respell(&z).expect("re-spell"), src, "the archive did not re-spell");
    }

    /// the recipe survives its own serialisation, and the values come back
    /// split the way they went in
    #[test]
    fn the_recipe_round_trips() {
        let a = raw_deflate(b"alpha alpha alpha alpha alpha");
        let b = raw_deflate(&b"beta".repeat(300));
        let src = build_zip(&[(&a, 8), (&b, 8)]);
        let mut z = peel(&src).expect("peel");
        let blob = blob(&z);
        let values = take_values(&mut z);
        assert_eq!(values_len(&blob).expect("values_len"), values.len());
        let mut back = from_blob(&blob).expect("from_blob");
        back.values = values;
        expand(&mut back).expect("expand");
        assert_eq!(respell(&back).expect("re-spell"), src, "the rebuilt recipe did not re-spell");
    }

    /// an archive with nothing to peel is refused, not expanded
    #[test]
    fn an_all_stored_archive_is_refused() {
        let src = build_zip(&[(b"one", 0), (b"two", 0)]);
        assert!(!nominates(&src), "an all-stored ZIP must not be nominated");
        assert!(peel(&src).is_err(), "an all-stored ZIP has nothing to peel");
    }

    /// a hostile recipe must refuse rather than read past itself
    #[test]
    fn a_short_or_lying_recipe_refuses() {
        let a = raw_deflate(b"gamma gamma gamma gamma");
        let src = build_zip(&[(&a, 8)]);
        let good = blob(&peel(&src).expect("peel"));
        assert!(from_blob(&good[..good.len() - 1]).is_err(), "a truncated recipe must refuse");
        assert!(from_blob(&[1u8; HDR]).is_err(), "a header with no sections must refuse");
        let mut lying = good.clone();
        lying[1] = 0xFF; // a member count its tables cannot hold
        assert!(from_blob(&lying).is_err(), "a lying member count must refuse");
    }
}
