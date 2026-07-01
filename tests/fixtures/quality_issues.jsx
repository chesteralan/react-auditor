function doNothing() {
  // no-empty-blocks
  if (true) {}

  // no-empty-blocks
  while (false) {}
}

// no-commented-code
// function oldCode(x) { return x * 2; }

// prefer-early-return
function earlyReturn(x) {
  if (x > 0) {
    return 'positive';
  } else {
    return 'not positive';
  }
}

// consistent-return
function mixedReturn(x) {
  if (x > 0) {
    return x;
  }
  return;
}

// no-shadow
const shadowed = 1;
function shadowExample() {
  const shadowed = 2;
  return shadowed;
}

// max-params
function manyParams(a, b, c, d) {
  return a + b + c + d;
}

// complexity (score = 1 base + 6 if + 2 for + 2 while + 2 case = 13 > 10)
function highComplexity(x) {
  if (x > 0) { console.log('a'); }
  if (x > 1) { console.log('b'); }
  if (x > 2) { console.log('c'); }
  if (x > 3) { console.log('d'); }
  if (x > 4) { console.log('e'); }
  if (x > 5) { console.log('f'); }
  for (let i = 0; i < x; i++) { console.log(i); }
  for (let j = 0; j < x; j++) { console.log(j); }
  while (x > 0) { x--; }
  while (x < 0) { x++; }
  switch (x) {
    case 1: break;
    case 2: break;
    default: break;
  }
}

// no-deep-nesting (5 levels)
function deepNesting() {
  if (true) {
    for (let i = 0; i < 10; i++) {
      if (i > 0) {
        while (i < 5) {
          if (i === 3) {
            console.log('deep');
          }
        }
      }
    }
  }
}

// no-long-functions (62 lines)
function longFunction() {
  let a = 1; let b = 2; let c = 3; let d = 4;
  let e = 5; let f = 6; let g = 7; let h = 8;
  let i = 9; let j = 10; let k = 11; let l = 12;
  let m = 13; let n = 14; let o = 15; let p = 16;
  let q = 17; let r = 18; let s = 19; let t = 20;
  let u = 21; let v = 22; let w = 23; let x = 24;
  let y = 25; let z = 26; let aa = 27; let bb = 28;
  let cc = 29; let dd = 30; let ee = 31; let ff = 32;
  let gg = 33; let hh = 34; let ii = 35; let jj = 36;
  let kk = 37; let ll = 38; let mm = 39; let nn = 40;
  let oo = 41; let pp = 42;
}

// inline comment to pad
// inline comment to pad

// no-empty-blocks with for
function emptyLoop() {
  for (let i = 0; i < 10; i++) {}
}

export default deepNesting;
