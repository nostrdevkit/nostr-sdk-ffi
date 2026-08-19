from nostr_sdk import git_hash_version


def main():
    print(git_hash_version())

if __name__ == "__main__":
    main()