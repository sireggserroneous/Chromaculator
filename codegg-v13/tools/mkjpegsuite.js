/* node tools/mkjpegsuite.js -- build corpus-jpeg, the peel's conservation suite.
 *
 * Provenance, stated because a suite you cannot trace is not evidence:
 *  (a) every .jpg under C:\Windows\Web on this machine (Screen, 4K,
 *      Wallpaper/{Alienware,Spotlight,ThemeA..D}, touchkeyboard) -- 46 files,
 *      eight of them already PROGRESSIVE (the touchkeyboard set), several with
 *      odd dimensions (3839x2400, 3841x2400, 3840x2401, 2054x1155), two of them
 *      byte-identical to the corpus row wallpaper.jpg;
 *  (b) two generated here with Windows' own WIC JpegBitmapEncoder (see
 *      tools/mkjpegsuite.ps1): a 4:2:0 colour frame and a Gray8 single-component
 *      frame, both JFIF (APP0, no Adobe APP14) and both restart-free;
 *  (c) hand-built HOSTILES, each a named mutation of a real baseline file:
 *      truncations, a corrupt DHT, a 12-bit SOF, an arithmetic SOF, a marker
 *      injected into the entropy data, and a non-JPEG wearing the JPEG magic.
 *
 * Nothing here is thrown away by the suite: every file must either peel and
 * re-spell BYTE-EXACT or be cleanly refused and keep its bytes. */
const fs = require('fs'), path = require('path'), cp = require('child_process');
const here = path.dirname(__filename);
const root = path.join(here, '..');
const dst = path.join(root, 'corpus-jpeg');
fs.mkdirSync(dst, { recursive: true });

function walk(dir, out) {
  let ents = [];
  try { ents = fs.readdirSync(dir, { withFileTypes: true }); } catch (e) { return out; }
  for (const e of ents) {
    const p = path.join(dir, e.name);
    if (e.isDirectory()) walk(p, out);
    else if (/\.jpe?g$/i.test(e.name)) out.push(p);
  }
  return out;
}

// (a) the machine's own JPEGs
const found = walk('C:\\Windows\\Web', []).sort();
let n = 0;
for (const f of found) {
  const rel = path.relative('C:\\Windows\\Web', f).replace(/[\\/]/g, '_');
  fs.copyFileSync(f, path.join(dst, 'win_' + rel));
  n++;
}
console.log(`(a) ${n} JPEGs copied from C:\\Windows\\Web`);

// (b) the WIC pair
const ps = path.join(here, 'mkjpegsuite.ps1');
if (fs.existsSync(ps)) {
  cp.execSync(`pwsh -NoProfile -File "${ps}" "${dst}"`, { stdio: 'inherit' });
}

// (c) the hostiles, each derived from a named real file
function findSeg(b, marker) { // returns [start, len] of the segment payload
  let i = 2;
  while (i + 3 < b.length) {
    if (b[i] !== 0xFF) return null;
    const m = b[i + 1];
    if (m === 0xD8 || m === 0x01 || (m >= 0xD0 && m <= 0xD7)) { i += 2; continue; }
    if (m === 0xD9 || m === 0xDA) return null;
    const L = (b[i + 2] << 8) | b[i + 3];
    if (m === marker) return [i, L];
    i += 2 + L;
  }
  return null;
}
const base = fs.readFileSync(path.join(root, 'corpus-real', 'wallpaper.jpg'));
const small = fs.readFileSync(path.join(dst, 'win_Wallpaper_Spotlight_img50.jpg'));

// a truncation in the middle of the entropy data
fs.writeFileSync(path.join(dst, 'hostile_truncated_mid.jpg'), small.subarray(0, Math.floor(small.length * 0.6)));
// a truncation that eats only the EOI
fs.writeFileSync(path.join(dst, 'hostile_truncated_eoi.jpg'), small.subarray(0, small.length - 2));
// a corrupt DHT: one byte of the symbol list changed
{
  const b = Buffer.from(small);
  const seg = findSeg(b, 0xC4);
  if (!seg) throw new Error('no DHT in the sample');
  b[seg[0] + 4 + 20] ^= 0xFF;
  fs.writeFileSync(path.join(dst, 'hostile_corrupt_dht.jpg'), b);
}
// a DHT whose counts claim more symbols than the segment carries
{
  const b = Buffer.from(small);
  const seg = findSeg(b, 0xC4);
  b[seg[0] + 4 + 1] = 0xFF; // 255 codes of length 1
  fs.writeFileSync(path.join(dst, 'hostile_dht_overrun.jpg'), b);
}
// 12-bit samples
{
  const b = Buffer.from(small);
  const seg = findSeg(b, 0xC0);
  b[seg[0] + 4] = 12;
  fs.writeFileSync(path.join(dst, 'hostile_12bit_sof.jpg'), b);
}
// an arithmetic-coded frame (SOF9)
{
  const b = Buffer.from(small);
  const seg = findSeg(b, 0xC0);
  b[seg[0] + 1] = 0xC9;
  fs.writeFileSync(path.join(dst, 'hostile_arithmetic_sof9.jpg'), b);
}
// a marker injected into the entropy-coded data
{
  const b = Buffer.from(small);
  let i = 2;
  while (i + 3 < b.length) {
    if (b[i] === 0xFF && b[i + 1] === 0xDA) { i += 2 + ((b[i + 2] << 8) | b[i + 3]); break; }
    i += 2 + ((b[i + 2] << 8) | b[i + 3]);
  }
  b[i + 200] = 0xFF; b[i + 201] = 0xC4;
  fs.writeFileSync(path.join(dst, 'hostile_marker_in_scan.jpg'), b);
}
// the JPEG magic on something that is not a JPEG
{
  const junk = Buffer.alloc(40000);
  let st = 0x1489;
  for (let k = 0; k < junk.length; k++) { st = (st * 1103515245 + 12345) & 0x7fffffff; junk[k] = st & 0xff; }
  junk[0] = 0xFF; junk[1] = 0xD8; junk[2] = 0xFF; junk[3] = 0xE0;
  fs.writeFileSync(path.join(dst, 'hostile_magic_only.jpg'), junk);
}
// a baseline JPEG with its scan replaced by noise (tables intact, codes wrong)
{
  const b = Buffer.from(small);
  let i = 2;
  while (i + 3 < b.length) {
    if (b[i] === 0xFF && b[i + 1] === 0xDA) { i += 2 + ((b[i + 2] << 8) | b[i + 3]); break; }
    i += 2 + ((b[i + 2] << 8) | b[i + 3]);
  }
  let st = 0xACE;
  for (let k = i; k < b.length - 2; k++) { st = (st * 1103515245 + 12345) & 0x7fffffff; const v = st & 0xff; b[k] = v === 0xFF ? 0xFE : v; }
  fs.writeFileSync(path.join(dst, 'hostile_scan_noise.jpg'), b);
}
// the corpus row itself, so the suite covers the row the gate is about
fs.copyFileSync(path.join(root, 'corpus-real', 'wallpaper.jpg'), path.join(dst, 'corpus_wallpaper.jpg'));
console.log(`(c) hostiles built; corpus-jpeg now holds ${fs.readdirSync(dst).length} files`);
console.log(`base file for the hostiles: win_Wallpaper_Spotlight_img50.jpg (${small.length} B); the corpus row is ${base.length} B`);
