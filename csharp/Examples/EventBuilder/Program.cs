using Nostr.Sdk;

using var keys = Keys.Generate();
using var kind = Kind.FromStd(KindStandard.TextNote);

using var textNoteBuilder = new EventBuilder(kind, "Note from rust-nostr C# bindings");
using var textNote = textNoteBuilder.Finalize(keys);
Console.WriteLine(textNote.AsJson());

using var customKind = new Kind(1234);
using var customBuilder = new EventBuilder(customKind, "My custom content");
using var customEvent = customBuilder.Finalize(keys);
Console.WriteLine($"Event: {customEvent.AsJson()}");

using var publicKey = keys.PublicKey();
using var unsignedEvent = customBuilder.FinalizeUnsigned(publicKey);
using var pow = new SingleThreadPow();
using var minedEvent = unsignedEvent.Mine(pow, 8);
using var powEvent = minedEvent.Sign(keys);
Console.WriteLine($"POW event: {powEvent.AsJson()}");

using var unsigned = customBuilder.FinalizeUnsigned(publicKey);
Console.WriteLine($"Unsigned event: {unsigned.AsJson()}");
