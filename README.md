# Executor: A Rust CLI Tool to Execute/Search/Delete Files

Note: In order to execute files, the files themselves must be executable.

## Commentary

The goal of this project is for my personal learnings and utility. Thus, I did
my best to minimize the layers of abstractions through avoiding the use of libraries that
others have developed.

For example, there is a much better implementation of traversing through directories
than the one I wrote - that being what is available in the walkdir crate. Although I would have benefited
greatly if the goal of this project was the build a tool that was the most
production ready and efficient as possible. However, since this project mainly
focused on honing my programming acumen, it made sense not to use that crate -
despite it provided better tooling that what I was able to build.
