// index.js
import init, { encrypt, decrypt } from './pkg/sdes.js';

function showToast(msg) {
    let toast = document.createElement('div');
    toast.className = 'toast';

    const heading = document.createElement('strong');
    heading.textContent = 'Error';
    heading.style.display = 'block';
    heading.style.fontSize = '1.5em';
    heading.style.marginBottom = '0.2em';

    toast.appendChild(heading);
    toast.appendChild(document.createTextNode(msg));

    document.body.appendChild(toast);

    setTimeout(() => {
        toast.style.opacity = '0';
        setTimeout(() => document.body.removeChild(toast), 300);
    }, 2000);
}

function isValidBinary(str, length) {
    return str.length === length && /^[01]+$/.test(str);
}

function randomBits(length) {
    let bits = '';
    for (let i = 0; i < length; i++) {
        bits += Math.random() < 0.5 ? '0' : '1';
    }
    return bits;
}

async function run() {
    await init();

    document.getElementById('randomKeyBtn').onclick = () => {
        document.getElementById('key').value = randomBits(10);
    };

    document.getElementById('encryptBtn').onclick = () => {
        const pt = document.getElementById('plaintext').value;
        const key = document.getElementById('key').value;
        const resultBox = document.getElementById('encryptResult');

        if (!isValidBinary(pt, 8)) {
            showToast("Plaintext must be 8 bits (0 or 1).");
            resultBox.textContent = '';
            return;
        }
        if (!isValidBinary(key, 10)) {
            showToast("Key must be 10 bits (0 or 1).");
            resultBox.textContent = '';
            return;
        }

        const ptNum = parseInt(pt, 2);
        const keyNum = parseInt(key, 2);

        const ct = encrypt(ptNum, keyNum);
        const ctStr = ct.toString(2).padStart(8, '0');
        resultBox.textContent = ctStr;
    };

    document.getElementById('decryptBtn').onclick = () => {
        const ct = document.getElementById('ciphertext').value;
        const key = document.getElementById('key').value;
        const resultBox = document.getElementById('decryptResult');

        if (!isValidBinary(ct, 8)) {
            showToast("Ciphertext must be 8 bits (0 or 1).");
            resultBox.textContent = '';
            return;
        }
        if (!isValidBinary(key, 10)) {
            showToast("Key must be 10 bits (0 or 1).");
            resultBox.textContent = '';
            return;
        }

        const ctNum = parseInt(ct, 2);
        const keyNum = parseInt(key, 2);

        const pt = decrypt(ctNum, keyNum);
        const ptStr = pt.toString(2).padStart(8, '0');
        resultBox.textContent = ptStr;
    };
}

run();