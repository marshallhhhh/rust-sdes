// index.js
import init, { encrypt, decrypt } from './pkg/sdes.js';

async function run() {
    await init();

    document.getElementById('encryptBtn').onclick = () => {
        const pt = document.getElementById('plaintext').value;
        const key = document.getElementById('key').value;
        const ptNum = parseInt(pt, 2);
        const keyNum = parseInt(key, 2);

        const ct = encrypt(ptNum, keyNum);
        const ctStr = ct.toString(2).padStart(8, '0');
        document.getElementById('ciphertext').value = ctStr;
    };

    document.getElementById('decryptBtn').onclick = () => {
        const ct = document.getElementById('ciphertext').value;
        const key = document.getElementById('key').value;
        const ctNum = parseInt(ct, 2);
        const keyNum = parseInt(key, 2);

        const pt = decrypt(ctNum, keyNum);
        const ptStr = pt.toString(2).padStart(8, '0');
        document.getElementById('plaintext').value = ptStr;
    };
}

run();