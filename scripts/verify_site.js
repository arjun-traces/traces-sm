const fs = require('fs');
const path = require('path');

const docsDir = path.join(__dirname, '../docs');

function getFiles(dir, fileList = []) {
    const files = fs.readdirSync(dir);
    for (const file of files) {
        const filePath = path.join(dir, file);
        if (fs.statSync(filePath).isDirectory()) {
            if (file !== '_site') getFiles(filePath, fileList);
        } else {
            fileList.push(filePath);
        }
    }
    return fileList;
}

const files = getFiles(docsDir);
console.log(`Found ${files.length} files in docs/`);

let cdnMatches = 0;
let forbiddenMatches = 0;

for (const file of files) {
    if (file.endsWith('.html') || file.endsWith('.md')) {
        const content = fs.readFileSync(file, 'utf8');
        
        if (content.includes('cdn.tailwindcss.com')) {
            console.error(`ERROR: cdn.tailwindcss.com found in ${file}`);
            cdnMatches++;
        }
        
        const forbidden = ['formally verified', '100% compliant'];
        for (const term of forbidden) {
            if (content.toLowerCase().includes(term)) {
                console.error(`WARNING: Forbidden term "${term}" found in ${file}`);
                forbiddenMatches++;
            }
        }
    }
}

console.log(`Verification completed. CDN occurrences: ${cdnMatches}, Forbidden terms: ${forbiddenMatches}`);
