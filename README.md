# CCHUFF: A Huffman Compression Algorithm encoder / decoder written in Rust

Made to tackle [this](https://codingchallenges.fyi/challenges/challenge-huffman/) challenge from https://coding-challenges.fyi

Tried to optimize performance as much as possible but so far I settled on:

## Huffman Encoding/Decoding Performance Analysis

### Performance Table

| File Size | Encoding Time | Decoding Time |
| --------- | ------------- | ------------- |
| 1MB       | ~95ms         | ~46ms         |
| 10MB      | ~907ms        | ~417ms        |
| 100MB     | ~9.1s         | ~4.1s         |
| Average   | ~3.37s        | ~1.52s        |

### Additional Statistics

1. **Encoding to Decoding Time Ratio**: On average, encoding takes about 2.22 times longer than decoding.

2. **Scaling Factor**: The processing time increases roughly linearly with file size, with a factor of about 10x for each 10x increase in file size.

### Device Specifications

- **OS**: Fedora Linux 40 (Sway) x86_64
- **Host**: G3 3500
- **Kernel**: Linux 6.10.6-200.fc40.x86_64
- **CPU**: Intel(R) Core(TM) i5-10300H (8) @ 4.50 GHz
- **GPU 1**: NVIDIA GeForce GTX 1650 Ti Mobile [Discrete]
- **GPU 2**: Intel UHD Graphics @ 1.05 GHz [Integrated]
- **Memory**: 15.41 GiB
- **Disk**: 69.01 GiB (btrfs)

### Observations

- The Huffman algorithm shows consistent performance across different file sizes, with encoding and decoding times scaling proportionally to the input size.
- Encoding is consistently more time-consuming than decoding, which is expected due to the nature of the Huffman algorithm.
- The linear scaling suggests good algorithmic efficiency, making it suitable for a wide range of file sizes.

### Conclusion

The Huffman encoding/decoding implementation demonstrates predictable and efficient performance characteristics. It handles larger files well, with a linear increase in processing time relative to file size. The faster decoding time is particularly beneficial for applications where data is encoded once but decoded multiple times.
