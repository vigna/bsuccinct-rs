`minimum_redundancy` is the Rust library by Piotr Beling to encode and decode data
with binary or non-binary Huffman coding.

The library can construct and concisely represent optimal prefix (minimum-redundancy) coding
whose codewords are of length divisible by a given number of bits (1-bit, 2-bits, ...).

The library uses modified Huffman algorithm, with ideas from papers:
- A. Brodnik, S. Carlsson, *Sub-linear decoding of Huffman Codes Almost In-Place*, 1998
- A. Moffat, J. Katajainen, *In-Place Calculation of Minimum-Redundancy Codes*.
  In: Akl S.G., Dehne F., Sack JR., Santoro N. (eds) Algorithms and Data Structures.
  WADS 1995. Lecture Notes in Computer Science, vol 955. Springer, Berlin, Heidelberg.
  <https://doi.org/10.1007/3-540-60220-8_79>

# Example
```rust
use minimum_redundancy::{Coding, Code};
use maplit::hashmap;

// Construct coding with 1 bit per fragment for values 'a', 'b', 'c',
// whose frequencies of occurrence are 100, 50, 10 times, respectively.
let huffman = Coding::from_frequencies_bits_per_fragment(hashmap!('a' => 100, 'b' => 50, 'c' => 10), 1);
// We expected the following Huffman tree:
//  /  \
// /\  a
// bc
// and the following code assignment: a -> 1, b -> 00, c -> 01
assert_eq!(huffman.codes_for_values(), hashmap!(
                'a' => Code{bits: 0b1, fragments: 1, bits_per_fragment: 1},
                'b' => Code{bits: 0b00, fragments: 2, bits_per_fragment: 1},
                'c' => Code{bits: 0b01, fragments: 2, bits_per_fragment: 1}
               ));
let mut decoder_for_a = huffman.decoder();
assert_eq!(decoder_for_a.consume(1), DecodingResult::Value(&'a'));
let mut decoder_for_b = huffman.decoder();
assert_eq!(decoder_for_b.consume(0), DecodingResult::Incomplete);
assert_eq!(decoder_for_b.consume(0), DecodingResult::Value(&'b'));
let mut decoder_for_c = huffman.decoder();
assert_eq!(decoder_for_c.consume(0), DecodingResult::Incomplete);
assert_eq!(decoder_for_c.consume(1), DecodingResult::Value(&'c'));
assert_eq!(huffman.total_fragments_count(), 5);
assert_eq!(huffman.values.as_ref(), ['a', 'b', 'c']);
```