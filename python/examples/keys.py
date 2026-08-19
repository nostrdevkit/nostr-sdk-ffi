from nostr_sdk import Keys


def main():
    keys = Keys.generate()
    print(keys.public_key().to_bech32())


if __name__ == "__main__":
    main()
