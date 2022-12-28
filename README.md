# rust-icao-alphabet
Library for encding a text into ICAO or NATO alphabet. It is usually used for spelling the callsign in radio communications.  
The alphabet begins with: Alfa, Bravo, Charlie, Delta, Echo, Foxtrot, ...

Each uppercase letter and digit is encoded according to the alphabet table. Spaces in the input text are not encoded.

```rust
// Create ICAO alphabet
let icao = Icao::new();

// Encode callsign
let callsign = icao.convert("LH26EDF").unwrap();
```

The output of callsign is:
```
"Lima Hotel Two Six Echo Delta Foxtrot "
```

## References
ICAO or NATO alphabet https://en.wikipedia.org/wiki/NATO_phonetic_alphabet  