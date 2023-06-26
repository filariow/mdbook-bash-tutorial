# mdbook bash tutorial

⚠️ this preprocessor is not production ready, use it with caution ⚠️

The Bash Tutorial mdbook preprocessor allows you to import a Bash script in your book.
Scripts remain easily testable and are embedded in the book at build time.

## Example

1. `chapter-1.md`
    ```markdown
    # Chapter 1

    {{#tutorial ./example/bash-script.sh}}
    ```
1. `example/bash-script.sh`
    ```bash
    #!/bin/bash

    ## Title
    echo "command"
    sleep 1000  # mdbash: skip-line

    ## Title-2
    echo "command-2"
    ```
1. Built chapter
    ````markdown
    # Chapter 1

    1. Title
    ```console
    echo "command"
    ```
    2. Title-2
    ```console
    echo "command-2"
    ```
    ````

