//! peel.rs -- THE PEEL FRAME (v13-M1, WS-F), generic, before any format.
//!
//! A peel is a length-changing, structure-parsing filter that emits
//! `(recipe, values)`. The recipe is everything needed to re-spell exactly and
//! is modelled on its own; the values are what the file actually SAYS and are
//! modelled by whatever model can read them. `restore` re-spells
//! `values + recipe -> the original bytes` and the FNV-64 of the original gates
//! it, as it gates everything else.
//!
//! THE LAW OF THE PEEL (charter plan, and it is a conservation law):
//!  1. A peel is a bijection or it is not used. `arm()` re-encodes its own
//!     output and compares against the original bytes BEFORE anything is
//!     written; one byte off and it returns None and the raw bytes go through
//!     the ordinary pipeline. The decoder only ever sees peels the encoder
//!     proved invertible on this exact input.
//!  2. The recipe rides inside and is judged with the values, by argmin on the
//!     ARMORED total -- never on inner bytes (v11-M8's kernel32 lesson).
//!  3. Refuse, do not guess: every refusal carries its reason.
//!  4. Every peel rides the write-time round-trip law (main.rs).
//!
//! The container: `MODEL_PEEL` in the header's model byte, and the payload
//! opens with a 15-byte preamble -- peel_id, the recipe's model and lengths,
//! the values' model and length -- followed by the recipe stream and then the
//! values stream. ONE constant, `PEEL_MAX`, bounds the id space and BOTH sides
//! read it (v12-M3 shipped an unreadable artifact for eleven minutes because an
//! id ceiling was written twice; it is written once here).
//!
//! v13-M2 adds the second peel and with it a second SHAPE of peel. The JPEG's
//! values are coefficients and only jcoef.rs can read them; the deflate peel's
//! values are ORDINARY BYTES -- the inflated file -- and the whole roster can.
//! `values_are_bytes` is the one place that distinction lives, and both
//! directions read it.

use crate::deflate;
use crate::jcoef;
use crate::jpeg;

/// no peel
pub const PEEL_NONE: u8 = 0;
/// baseline JPEG: the Huffman spelling off, the coefficients underneath
pub const PEEL_JPEG: u8 = 1;
/// deflate (gzip, zlib, PNG IDAT, a bare stream): the parse off, the bytes
/// underneath
pub const PEEL_DEFLATE: u8 = 2;
/// the highest peel id this build can read. ONE constant, both sides.
pub const PEEL_MAX: u8 = PEEL_DEFLATE;

/// the preamble that opens a MODEL_PEEL payload
pub const PREAMBLE: usize = 15;

/// which peel, if any, this file's own magic nominates. Nomination is never a
/// decision: the parse may still refuse, and the trial may still prefer the raw
/// form.
pub fn nominate(src: &[u8]) -> u8 {
    if src.len() >= 4 && src[0] == 0xFF && src[1] == 0xD8 && src[2] == 0xFF {
        return PEEL_JPEG;
    }
    if deflate::nominates(src) {
        return PEEL_DEFLATE;
    }
    PEEL_NONE
}

/// one member of a container, as its own LAYOUT declares it (v13-M3d, S3a).
/// The reading is `atlas.html:355-356` and `461-462`: compute which member owns
/// an offset from arithmetic, and return NULL, not a guess, when the offset
/// lands outside any member.
pub struct Member {
    pub off: usize,
    pub len: usize,
    /// the container's own method number (8 = deflate, 0 = stored)
    pub method: u16,
    pub name: String,
}

/// THE OFFSET-TO-MEMBER PROBER. Reads a ZIP's central directory -- which is the
/// container's own arithmetic, not a scan -- and returns every member's body
/// offset and length. `None` means "this is not a container I can read", never
/// "there are no members": a guess here is exactly the thing the reading
/// forbids, and `looks_like_deflate` returning true-with-no-check on block type
/// 1 is what a guess costs (9 of the sealed twenty already pay for a peel
/// attempt that fails).
pub fn members(src: &[u8]) -> Option<Vec<Member>> {
    // the End Of Central Directory record, hunted from the tail (its comment
    // may be up to 65,535 B, so the window is bounded and so is the hunt)
    let n = src.len();
    if n < 22 {
        return None;
    }
    let lo = n.saturating_sub(22 + 65_535);
    let mut eocd = None;
    let mut i = n - 22;
    loop {
        if &src[i..i + 4] == b"PK\x05\x06" {
            eocd = Some(i);
            break;
        }
        if i == lo {
            break;
        }
        i -= 1;
    }
    let e = eocd?;
    let count = u16::from_le_bytes([src[e + 10], src[e + 11]]) as usize;
    let cdsize = u32::from_le_bytes([src[e + 12], src[e + 13], src[e + 14], src[e + 15]]) as usize;
    let cdoff = u32::from_le_bytes([src[e + 16], src[e + 17], src[e + 18], src[e + 19]]) as usize;
    if cdoff.checked_add(cdsize)? > n {
        return None;
    }
    let mut out = Vec::with_capacity(count);
    let mut at = cdoff;
    while at + 46 <= cdoff + cdsize {
        if &src[at..at + 4] != b"PK\x01\x02" {
            return None;
        }
        let method = u16::from_le_bytes([src[at + 10], src[at + 11]]);
        let csize = u32::from_le_bytes([src[at + 20], src[at + 21], src[at + 22], src[at + 23]]) as usize;
        let nlen = u16::from_le_bytes([src[at + 28], src[at + 29]]) as usize;
        let elen = u16::from_le_bytes([src[at + 30], src[at + 31]]) as usize;
        let clen = u16::from_le_bytes([src[at + 32], src[at + 33]]) as usize;
        let lho = u32::from_le_bytes([src[at + 42], src[at + 43], src[at + 44], src[at + 45]]) as usize;
        let name = String::from_utf8_lossy(src.get(at + 46..at + 46 + nlen)?).into_owned();
        // the LOCAL header carries its own name and extra lengths, and they
        // are NOT the central directory's: the body offset is arithmetic on
        // the local header, which is the whole point of the reading
        if lho + 30 > n || &src[lho..lho + 4] != b"PK\x03\x04" {
            return None;
        }
        let lnlen = u16::from_le_bytes([src[lho + 26], src[lho + 27]]) as usize;
        let lelen = u16::from_le_bytes([src[lho + 28], src[lho + 29]]) as usize;
        let off = lho + 30 + lnlen + lelen;
        if off.checked_add(csize)? > n {
            return None;
        }
        out.push(Member { off, len: csize, method, name });
        at += 46 + nlen + elen + clen;
    }
    if out.len() != count {
        return None;
    }
    Some(out)
}

/// which member, if any, owns this offset -- and NULL, not a guess, when it
/// lands outside every one of them
pub fn owner(ms: &[Member], off: usize) -> Option<usize> {
    ms.iter().position(|m| off >= m.off && off < m.off + m.len)
}

/// what a peel produced, before it is modelled
pub struct Peeled {
    pub id: u8,
    pub recipe: Vec<u8>,
    pub jpeg: Option<jpeg::Jpeg>,
    pub deflate: Option<deflate::Deflate>,
    /// the raw weight of the values: for the JPEG, 2 B per coefficient (the
    /// dump v12 weighed); for deflate, the inflated byte count
    pub values_raw_len: usize,
}

/// do this peel's values ride the ORDINARY model ladder (the roster, a model
/// byte from main.rs), or its own values model? One answer, read by the arm and
/// by the restore.
pub fn values_are_bytes(id: u8) -> bool {
    id == PEEL_DEFLATE
}

/// peel `src`, or say why not
pub fn peel(src: &[u8], id: u8) -> Result<Peeled, String> {
    match id {
        PEEL_JPEG => {
            let j = jpeg::peel(src)?;
            let recipe = jpeg::recipe_bytes(&j);
            let values_raw_len = jcoef::raw_len(&j);
            Ok(Peeled { id, recipe, jpeg: Some(j), deflate: None, values_raw_len })
        }
        PEEL_DEFLATE => {
            let d = deflate::peel(src)?;
            let recipe = deflate::blob(&d);
            let values_raw_len = d.values.len();
            Ok(Peeled { id, recipe, jpeg: None, deflate: Some(d), values_raw_len })
        }
        _ => Err(format!("peel id {} is not a peel", id)),
    }
}

/// what this peel found, for the trace
pub fn describe(p: &Peeled) -> String {
    match (p.jpeg.as_ref(), p.deflate.as_ref()) {
        (Some(j), _) => j.describe(),
        (_, Some(d)) => d.describe(),
        _ => "a peel with no parse".into(),
    }
}

/// re-spell a peel back to the original bytes
pub fn respell(p: &Peeled) -> Result<Vec<u8>, String> {
    match p.id {
        PEEL_JPEG => jpeg::respell(p.jpeg.as_ref().ok_or("peel: no JPEG parse")?),
        PEEL_DEFLATE => deflate::respell(p.deflate.as_ref().ok_or("peel: no deflate parse")?),
        _ => Err(format!("peel id {} is not a peel", p.id)),
    }
}

/// the values as ORDINARY BYTES, taken out of the parse so the roster can model
/// them. Only ever called after THE LAW's re-spell has already run.
pub fn take_values(p: &mut Peeled) -> Vec<u8> {
    match p.deflate.as_mut() {
        Some(d) => std::mem::take(&mut d.values),
        None => Vec::new(),
    }
}

/// the values, modelled by the peel's OWN model. Returns (stream, model byte).
///
/// M1 files ONE values model per peel: the JPEG coefficient model (jcoef.rs).
/// The generic byte arms are NOT run on the coefficient dump -- v12 measured
/// that road and printed the number (xz -9 2,192,684 B, our MIX12 arm
/// 2,208,961 B, both HEAVIER than the JPEG's own 1,602,311 B of Huffman on
/// wallpaper.jpg). Running a 55 MB dump through the roster to re-measure a
/// known loss would cost minutes per file and buy nothing; the reading is
/// printed instead of repeated.
pub fn encode_values(p: &mut Peeled, values_model: &mut u8) -> Vec<u8> {
    match p.id {
        PEEL_JPEG => {
            *values_model = MODEL_JCOEF;
            jcoef::encode(p.jpeg.as_mut().expect("JPEG parse"))
        }
        _ => unreachable!("encode_values on a peel whose values are bytes"),
    }
}

/// the values model byte: the JPEG coefficient model. It lives in the peel
/// preamble, never in the container's own model byte.
pub const MODEL_JCOEF: u8 = 25;
/// the deflate RECIPE model byte: the four streams, each through the roster in
/// its own language. It lives in the preamble's recipe-model slot and is
/// dispatched by main.rs's `restore_bytes`, like any other model.
pub const MODEL_DRECIPE: u8 = 26;

/// values stream + recipe -> the original bytes, for a peel with its OWN values
/// model
pub fn decode_and_respell(id: u8, recipe: &[u8], values_model: u8, stream: &[u8]) -> Result<Vec<u8>, String> {
    if id == PEEL_NONE || id > PEEL_MAX {
        return Err(format!("peel id {} is beyond this build's ceiling {}", id, PEEL_MAX));
    }
    match id {
        PEEL_JPEG => {
            if values_model != MODEL_JCOEF {
                return Err(format!("the JPEG peel cannot read values model {}", values_model));
            }
            let mut j = jpeg::from_recipe(recipe)?;
            jcoef::decode(stream, &mut j)?;
            jpeg::respell(&j)
        }
        _ => Err(format!("peel id {} takes its values as bytes, not as a values model", id)),
    }
}

/// recipe + the values ALREADY RESTORED AS BYTES -> the original bytes
pub fn respell_bytes(id: u8, recipe: &[u8], values: Vec<u8>) -> Result<Vec<u8>, String> {
    if id == PEEL_NONE || id > PEEL_MAX {
        return Err(format!("peel id {} is beyond this build's ceiling {}", id, PEEL_MAX));
    }
    match id {
        PEEL_DEFLATE => {
            let mut d = deflate::from_blob(recipe)?;
            if values.len() as u64 != deflate::layout(recipe)?.values_len {
                return Err(format!(
                    "the deflate recipe expects {} inflated bytes, the values restored {}",
                    deflate::layout(recipe)?.values_len,
                    values.len()
                ));
            }
            d.values = values;
            // v14-N3b: a version-2 recipe carries no parse -- a matcher
            // rebuilds it from the values that just arrived. A version-1
            // recipe already has one and this is a no-op.
            deflate::expand(&mut d)?;
            deflate::respell(&d)
        }
        _ => Err(format!("peel id {} does not take its values as bytes", id)),
    }
}

/// the preamble, written
pub fn write_preamble(id: u8, recipe_model: u8, recipe_raw: usize, recipe_stream: usize, values_model: u8, values_raw: usize) -> [u8; PREAMBLE] {
    let mut b = [0u8; PREAMBLE];
    b[0] = id;
    b[1] = recipe_model;
    b[2..6].copy_from_slice(&(recipe_raw as u32).to_le_bytes());
    b[6..10].copy_from_slice(&(recipe_stream as u32).to_le_bytes());
    b[10] = values_model;
    b[11..15].copy_from_slice(&(values_raw as u32).to_le_bytes());
    b
}
/// the preamble, read back: (id, recipe_model, recipe_raw, recipe_stream, values_model, values_raw)
pub fn read_preamble(b: &[u8]) -> Result<(u8, u8, usize, usize, u8, usize), String> {
    if b.len() < PREAMBLE {
        return Err("peel payload shorter than its preamble".into());
    }
    let id = b[0];
    if id == PEEL_NONE || id > PEEL_MAX {
        return Err(format!("peel id {} is beyond this build's ceiling {}", id, PEEL_MAX));
    }
    Ok((
        id,
        b[1],
        u32::from_le_bytes(b[2..6].try_into().unwrap()) as usize,
        u32::from_le_bytes(b[6..10].try_into().unwrap()) as usize,
        b[10],
        u32::from_le_bytes(b[11..15].try_into().unwrap()) as usize,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// a minimal two-member ZIP, built here so the prober is tested against a
    /// layout this file knows the answer to. The LOCAL header carries its own
    /// name and extra lengths and they are deliberately NOT the central
    /// directory's -- that difference is the whole reason the body offset is
    /// arithmetic on the local header, and a prober that reads the central
    /// directory's lengths would land in the wrong place.
    fn tiny_zip() -> (Vec<u8>, Vec<(usize, usize)>) {
        let names: [&[u8]; 2] = [b"a.txt", b"dir/b.bin"];
        let bodies: [&[u8]; 2] = [b"hello", b"0123456789"];
        let mut z: Vec<u8> = Vec::new();
        let mut lho = Vec::new();
        let mut want = Vec::new();
        for i in 0..2 {
            lho.push(z.len());
            z.extend_from_slice(b"PK\x03\x04");
            z.extend_from_slice(&[20, 0, 0, 0, 0, 0, 0, 0, 0, 0]); // version..time
            z.extend_from_slice(&0u32.to_le_bytes()); // crc
            z.extend_from_slice(&(bodies[i].len() as u32).to_le_bytes());
            z.extend_from_slice(&(bodies[i].len() as u32).to_le_bytes());
            z.extend_from_slice(&(names[i].len() as u16).to_le_bytes());
            z.extend_from_slice(&4u16.to_le_bytes()); // LOCAL extra: four bytes
            z.extend_from_slice(names[i]);
            z.extend_from_slice(&[0, 0, 0, 0]);
            want.push((z.len(), bodies[i].len()));
            z.extend_from_slice(bodies[i]);
        }
        let cdoff = z.len();
        for i in 0..2 {
            z.extend_from_slice(b"PK\x01\x02");
            z.extend_from_slice(&[20, 0, 20, 0, 0, 0, 0, 0, 0, 0, 0, 0]); // ..method 0
            z.extend_from_slice(&0u32.to_le_bytes()); // crc
            z.extend_from_slice(&(bodies[i].len() as u32).to_le_bytes());
            z.extend_from_slice(&(bodies[i].len() as u32).to_le_bytes());
            z.extend_from_slice(&(names[i].len() as u16).to_le_bytes());
            z.extend_from_slice(&0u16.to_le_bytes()); // CENTRAL extra: none
            z.extend_from_slice(&0u16.to_le_bytes()); // comment
            z.extend_from_slice(&[0u8; 8]); // disk(2) + internal(2) + external(4) = 8, and
            // NOT ten: the local-header offset lives at byte 42 of this record
            z.extend_from_slice(&(lho[i] as u32).to_le_bytes());
            z.extend_from_slice(names[i]);
        }
        let cdsize = z.len() - cdoff;
        z.extend_from_slice(b"PK\x05\x06");
        z.extend_from_slice(&[0, 0, 0, 0]);
        z.extend_from_slice(&2u16.to_le_bytes());
        z.extend_from_slice(&2u16.to_le_bytes());
        z.extend_from_slice(&(cdsize as u32).to_le_bytes());
        z.extend_from_slice(&(cdoff as u32).to_le_bytes());
        z.extend_from_slice(&0u16.to_le_bytes());
        (z, want)
    }

    #[test]
    fn the_prober_reads_a_layout_and_says_null_outside_it() {
        let (z, want) = tiny_zip();
        let ms = members(&z).expect("a ZIP layout");
        assert_eq!(ms.len(), 2);
        for (i, (off, len)) in want.iter().enumerate() {
            assert_eq!(ms[i].off, *off, "member {} body offset", i);
            assert_eq!(ms[i].len, *len, "member {} body length", i);
        }
        assert_eq!(ms[0].name, "a.txt");
        assert_eq!(ms[1].name, "dir/b.bin");
        // owner() is arithmetic, and it returns NULL rather than guessing
        assert_eq!(owner(&ms, ms[0].off), Some(0));
        assert_eq!(owner(&ms, ms[1].off + ms[1].len - 1), Some(1));
        assert_eq!(owner(&ms, 0), None, "the local header belongs to no member");
        assert_eq!(owner(&ms, z.len() - 1), None, "the EOCD belongs to no member");
    }

    #[test]
    fn the_prober_refuses_what_it_cannot_read() {
        assert!(members(b"not a container at all").is_none());
        assert!(members(&[0xFFu8; 4096]).is_none());
        let (mut z, _) = tiny_zip();
        let n = z.len();
        z[n - 6] = 0xFF; // a central-directory offset past the end of the file
        assert!(members(&z).is_none(), "a layout that does not fit must be refused, not guessed");
    }
}
