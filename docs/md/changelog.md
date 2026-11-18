## Changelog

### v

#### Performance Optimizations

- **Replaced regular expressions with native string operations:** Removed regex-based matching for command-line arguments, remote missing-file detection, and yes/no confirmation prompts. These operations now use simpler string comparisons, including case-insensitive comparisons where appropriate, reducing unnecessary overhead.
- **Optimized file comparison:** Replaced the process of hashing both downloaded and local hosts data just to determine whether they differ with a direct byte-for-byte comparison.
- **Made MD5 calculation conditional:** MD5 checksums are now calculated only when the downloaded content differs from the existing hosts file, avoiding unnecessary hashing when no update is needed.
- **Streamlined argument handling:** Simplified command-line argument processing by skipping the executable argument directly, reducing intermediate state, and using more straightforward collection handling.
- **Reduced unnecessary allocations:** Added capacity preallocation where appropriate and simplified temporary data structures and variables.
- **Simplified URL and path construction:** Reduced intermediate string handling when constructing download URLs and alternate-list paths.

#### Code Cleanup

- **Replaced `recolored` usage in `main.rs`:** Removed its colorization usage in favor of lightweight ANSI escape-code helpers, preserving colored and bold terminal output without relying on the additional abstraction.
- **Removed unused imports and redundant logic:** Cleaned up unnecessary declarations and simplified operations that were more complicated than needed.
- **Simplified conditional logic:** Reduced redundant checks and intermediate variables while retaining the existing behavior.

##### v1.0.1793744309

- Changed Rust build to statically link openssl. This is for the sake of the Linux build and the only alterntive is to package in a container which I do not want to at this time.

### v1.0.109134

- Changed the versioning system.

### v1.0.0

- Initial release (see [Project README](../../) for details).

## Future Changes

### v1.1.* - planned

- Optimization/performance fixes.
- Complete change/update to workflow build system.

### v2.0.* - planned

- Changing delivery system to libcurl (currently JSDELIVR).