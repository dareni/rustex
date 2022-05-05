# Rust Language Examples

Examples from rust tutorials.

# Rust VIM Setup

## Install Rust
```
rustup +nightly  component add rust-analyzer-preview
rustup run nightly rust-analyzer
cargo install rusty-tags
```
- Make sure rust-analyzer is in the path.

## Install Linter
```
git clone https://github.com/dense-analysis/ale.git ~/.vim/pack/git-plugins/start/ale
git clone https://github.com/vim-airline/vim-airline ~/.vim/pack/git-plugins/start/vim-airline
```

###Load help tags in vim:
>  :helpt ~/.vim/pack/git-plugins/start/ale/doc

###Configure linter in _vimrc
```
let g:ale_linters = {'rust': ['analyzer', 'cargo', 'rls', 'rustc']}
let g:ale_fixers = { 'rust': ['rustfmt', 'trim_whitespace', 'remove_trailing_lines'] }
let g:ale_completion_enabled = 1
let g:ale_sign_column_always = 1
let g:airline#extensions#ale#enabled = 1
"Only lint project files
let g:ale_pattern_options = {'\/opt\/dev\/*.rs$': {'ale_enabled': 0}}
"Enable quickfix
let g:ale_set_loclist = 1
let g:ale_set_quickfix = 1
let g:ale_open_list = 1
let g:ale_keep_list_window_open = 1
let g:ale_list_window_size = 5
let g:ale_pattern_options_enabled = 1
let g:ale_hover_preview = 1

"Activate Ale autocomplete ie ctrl-x ctrl-a in insert mode.
imap <C-A> <Plug>(ale_complete)

map \gd :ALEGoToDefinition
map \cl :cexpr []
map \da :let ale_enabled=0

map \cr :!clear; cargo run
map \ct :!clear; cargo test -- --nocapture
map \ft :%!rustfmt
```
## Configure rusty-tags in _vimrc
```
function! RustyTags()
  call system("rusty-tags vi")
  let l:tpath=system("pwd")
  let l:tpath= trim(l:tpath)."/rusty-tags.vi"
  let &tags=l:tpath
endfunction
"run rusty-tags
map \rt :call RustyTags() <RETURN>
```

## Configure Debugger (gdb) in _vimrc
```
function! GdbSetup()
  set mouse=a
  let g:termdebug_popup = 0
  let g:termdebug_wide = 163
  packadd termdebug
endfunction

map \rg :call GdbSetup()
```

### Debug Workflow example:
1. Open rust source file.  `vim src/main.rs`
2. In vim press `\rg` to run gdb setup function GdbSetup()
3. Run the debugger with the executable `:TermDebug target/debug/prog`
    The gdb will now be running with the target executable.
4. In the source window navigate to the first line and type `:Break`.
    A breakpoint should now be set, :Clear to remove it.
5. Type `:Run`
    Program execution point will be highlighted on the breakpoint line.
6. Use the <Next> or (`:Over` command) button to step through the code.
7. Mouse over variables to examine values
8. Reload the binary from gdb ` file target/debug/prog`

### Debug Rust Unit Test
1. Open the rust source file with the test 'vim src/lib.rs'
2. In vim press `\rg` to run gdb setup function GdbSetup()
3. Run the test eg `cargo test -- test_is_pid`  gives output:
```
    Finished test [unoptimized + debuginfo] target(s) in 0.01s
     Running unittests (target/debug/deps/clicker-514f7f3c89752650)

running 1 test
out:false
test tests::test_is_pid ... ok

```
4. Run the debugger with the executable `:TermDebug target/debug/deps/clicker-514f7f3c89752650`
5. Continue with debug workflow.

## Useful vim commands:
### Vim quickfix
> :cc :cn :cp
### Clear quickfix list
> :cexpr []
### Vim Help
```
:help Termdebug
:help using<Plug>
```
