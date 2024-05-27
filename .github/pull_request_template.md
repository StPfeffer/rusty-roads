# Pull Request Template

## Description

Please include a summary of the change and which issue (if any) is fixed.
A brief description of the algorithm and your implementation method can be helpful too. If the implemented method/algorithm is not so
well-known, it would be helpful to add a link to an article explaining it with more details.

## Type of change

- [ ] Bug fix (non-breaking change which fixes an issue)
- [ ] New feature (non-breaking change which adds functionality)
- [ ] Breaking change (fix or feature that would cause existing functionality to not work as expected)

## Checklist

- [ ] I ran `cargo clippy --all -- -D warnings` just before my last commit and fixed any issue that was found.
- [ ] I ran `cargo fmt --all -- --check` just before my last commit.
- [ ] I ran `cargo test` just before my last commit and all tests passed.
- [ ] I added my algorithm to the corresponding `mod.rs` file within its own folder, and in any parent folder(s).
