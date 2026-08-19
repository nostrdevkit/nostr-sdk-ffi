from nostr_sdk import *


def main():
    keys = Keys.generate()

    # Build a text note
    builder = EventBuilder(Kind.from_std(KindStandard.TEXT_NOTE), "Note from rust-nostr Python bindings")
    event = builder.finalize(keys)
    print(event.as_json())

    # Build a custom event
    kind = Kind(1234)
    content = "My custom content"
    builder = EventBuilder(kind, content)

    # Sign with generic signer
    event = builder.finalize(keys)
    print(f"Event: {event.as_json()}")

    # POW
    unsigned = builder.finalize_unsigned(keys.public_key())
    event = unsigned.mine(SingleThreadPow(), 8).sign(keys)
    print(f"POW event: {event.as_json()}")

    # Build unsigned event
    unsigned = builder.finalize_unsigned(keys.public_key())
    print(f"Unsigned event: {unsigned.as_json()}")


if __name__ == '__main__':
    main()
