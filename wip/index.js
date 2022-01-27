const vdf = require('vdf-parser');

const text = `
Tes-_---_--
{
    "Test" ""
}
`.trim();

console.log(vdf.parse(text));