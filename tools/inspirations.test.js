/* node tools/inspirations.test.js — the filter on the attributions page.
 *
 * The catalogue is read out of the page's markup through querySelectorAll,
 * which the harness leaves empty, so the list itself cannot be exercised here.
 * The decision can be: keeps() takes a plain object and returns a boolean, and
 * that is where every rule the page claims actually lives. */
const {loadPage} = require(__dirname + "/domharness.js");
const {run} = loadPage(__dirname + "/../inspirations.html");
const ok = (c, m) => { if(!c) throw new Error("FAIL " + m); };

const item = (id, text, year) => JSON.stringify({id, text, year});
const keeps = (it, sec, q, before) => run(`keeps(${it}, ${JSON.stringify(sec)}, ${JSON.stringify(q)}, ${before})`);

const BOOTH = item("s-the-digits", "signed-digit representation booth, 1951", 1951);
const CONWAY = item("s-the-stalks", "on numbers and games conway, 1976", 1976);
const UNDATED = item("s-the-fold", "the anti-diagonal fold, held loosely", null);

/* 1. no filter keeps everything */
{
  for(const it of [BOOTH, CONWAY, UNDATED])
    ok(keeps(it, "", "", 2025), "an empty filter dropped an item");
  console.log(`  an empty filter keeps every item`);
}

/* 2. the section dropdown keeps only its own section */
{
  ok(keeps(BOOTH, "s-the-digits", "", 2025), "section filter dropped its own item");
  ok(!keeps(BOOTH, "s-the-stalks", "", 2025), "section filter kept a foreign item");
  ok(!keeps(UNDATED, "s-the-digits", "", 2025), "section filter kept a foreign undated item");
  console.log(`  the section dropdown keeps only that section`);
}

/* 3. search is a substring of the item's text, already lowercased */
{
  ok(keeps(CONWAY, "", "conway", 2025), "search missed a name in the text");
  ok(keeps(CONWAY, "", "games", 2025), "search missed a word in the middle");
  ok(!keeps(CONWAY, "", "avizienis", 2025), "search kept an item it does not match");
  ok(keeps(CONWAY, "", "", 2025), "an empty search should not filter");
  console.log(`  search keeps an item when the query appears in its text`);
}

/* 4. the year bar. This is the rule most easily got wrong: an item with no
      year must survive every position of the bar, or dragging it wipes out
      every undated attribution on the page. */
{
  ok(keeps(BOOTH, "", "", 1951), "1951 should survive 'before 1951'");
  ok(!keeps(BOOTH, "", "", 1950), "1951 should not survive 'before 1950'");
  ok(keeps(CONWAY, "", "", 2025), "1976 should survive 'before 2025'");
  ok(!keeps(CONWAY, "", "", 1960), "1976 should not survive 'before 1960'");
  for(const y of [1800, 1900, 1951, 2025])
    ok(keeps(UNDATED, "", "", y), `an undated item was hidden at before=${y}`);
  console.log(`  the year bar filters dated items and never hides undated ones`);
}

/* 5. the rules compose — all three at once, and any one failing is enough */
{
  ok(keeps(BOOTH, "s-the-digits", "booth", 1960), "all three matching should keep");
  ok(!keeps(BOOTH, "s-the-digits", "booth", 1900), "the year alone should be enough to drop");
  ok(!keeps(BOOTH, "s-the-stalks", "booth", 1960), "the section alone should be enough to drop");
  ok(!keeps(BOOTH, "s-the-digits", "conway", 1960), "the search alone should be enough to drop");
  console.log(`  the three rules compose, and any one of them can drop an item`);
}

/* 6. the page holds up with no catalogue at all, which is exactly the state
      the harness puts it in -- an empty chart and an honest readout, no throw */
{
  ok(run(`ITEMS.length`) === 0, "the harness unexpectedly found items");
  run(`draw();`);
  ok(run(`$_ = document.getElementById("iout").textContent`).indexOf("No catalogue") >= 0,
     `an empty catalogue should say so, got "${run(`document.getElementById("iout").textContent`)}"`);
  console.log(`  an empty catalogue draws and reports itself rather than throwing`);
}

/* 7. the year regex only accepts plausible years, so a page number or a count
      in an item's text is not mistaken for a date */
{
  const re = /\b(1[6-9]\d\d|20[0-2]\d)\b/;
  for(const [text, want] of [["booth, 1951", "1951"], ["avizienis, 1961", "1961"],
                             ["the 20,000-pair test", null], ["ring 2^15", null],
                             ["conway 1976", "1976"], ["page 300", null]]){
    const m = text.match(re);
    const got = m ? m[1] : null;
    ok(got === want, `year in "${text}": got ${got}, want ${want}`);
  }
  console.log(`  the year pattern reads dates and ignores counts and page numbers`);
}

console.log("inspirations ok");
