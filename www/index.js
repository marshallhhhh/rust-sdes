// index.js
import init, { encrypt, decrypt } from './pkg/sdes.js';

function showToast(msg) {
    let toast = document.createElement('div');
    toast.className = 'toast';

    const heading = document.createElement('strong');
    heading.textContent = 'Error';
    heading.style.display = 'block';
    heading.style.fontSize = '1.3em';
    heading.style.marginBottom = '0.2em';

    toast.appendChild(heading);
    toast.appendChild(document.createTextNode(msg));

    document.body.appendChild(toast);

    setTimeout(() => {
        toast.style.opacity = '0';
        setTimeout(() => document.body.removeChild(toast), 300);
    }, 2000);
}

async function run() {
    await init();

    function isValidBinary(str, length) {
        return str.length === length && /^[01]+$/.test(str);
    }

    document.getElementById('encryptBtn').onclick = () => {
        const pt = document.getElementById('plaintext').value;
        const key = document.getElementById('key').value;

        if (!isValidBinary(pt, 8)) {
            showToast("Plaintext must be 8 bits (0 or 1).");
            return;
        }
        if (!isValidBinary(key, 10)) {
            showToast("Key must be 10 bits (0 or 1).");
            return;
        }

        const ptNum = parseInt(pt, 2);
        const keyNum = parseInt(key, 2);

        const ct = encrypt(ptNum, keyNum);
        const ctStr = ct.toString(2).padStart(8, '0');
        document.getElementById('ciphertext').value = ctStr;
    };

    document.getElementById('decryptBtn').onclick = () => {
        const ct = document.getElementById('ciphertext').value;
        const key = document.getElementById('key').value;

        if (!isValidBinary(ct, 8)) {
            showToast("Ciphertext must be 8 bits (0 or 1).");
            return;
        }
        if (!isValidBinary(key, 10)) {
            showToast("Key must be 10 bits (0 or 1).");
            return;
        }

        const ctNum = parseInt(ct, 2);
        const keyNum = parseInt(key, 2);

        const pt = decrypt(ctNum, keyNum);
        const ptStr = pt.toString(2).padStart(8, '0');
        document.getElementById('plaintext').value = ptStr;
    };
}

run();