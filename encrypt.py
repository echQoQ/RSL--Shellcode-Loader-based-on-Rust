import os
import base64
import hashlib
import argparse
from Crypto.Cipher import ARC4


def read_binary_file(file_path):
    try:
        with open(file_path, 'rb') as f:
            return f.read()
    except FileNotFoundError:
        print(f"Error: The file {file_path} does not exist.")
        return None


def save_encrypted_base64(file_path, b64_data):
    with open(file_path, 'wb') as f:
        f.write(b64_data)


def parse_args():
    p = argparse.ArgumentParser(description="Encrypt binary to new payload format")
    p.add_argument("-i", "--input", default="calc.bin", help="input binary file (default calc.bin)")
    p.add_argument("-o", "--output", default="src/encrypt.bin", help="output base64 file (default src/encrypt.bin)")
    p.add_argument("-m", "--method", default="rc4", choices=["rc4"],
                   help="encryption method (supported: rc4)")
    return p.parse_args()



def main():
    args = parse_args()

    input_file = args.input
    output_file = args.output

    data = read_binary_file(input_file)
    if data is None:
        return

    if args.method == "rc4":
        from Crypto.Cipher import ARC4
        key = os.urandom(32)  # 256-bit key
        cipher = ARC4.new(key)
        encrypted = cipher.encrypt(data)
        sha256 = hashlib.sha256()
        sha256.update(data)
        hash1 = sha256.digest()
        # 格式: [key(32)][hash(32)][encrypted...]
        final = key + hash1 + encrypted
    else:
        raise SystemExit(f"Unsupported --method: {args.method}")

    b64 = base64.b64encode(final)
    save_encrypted_base64(output_file, b64)
    print(f"Encrypted data (new format, method={args.method}) saved to {output_file}")


if __name__ == '__main__':
    main()

