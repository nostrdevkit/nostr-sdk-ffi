using Nostr.Sdk;

using var keys = Keys.Generate();
using var publicKey = keys.PublicKey();

Console.WriteLine(publicKey.ToBech32());
