# lintls

## Installing in Neovim

```lua
vim.api.nvim_create_autocmd({ "BufRead" }, {
  group = vim.api.nvim_create_augroup("lintls-bufread", { clear = true }),
  callback = function(_)
    if vim.fn.executable("lintls") ~= 0 then
      -- We found an executable for lintls.
      vim.lsp.set_log_level(vim.log.levels.INFO)
      vim.lsp.start({
        name = "lintls",
        cmd = { "lintls", vim.api.nvim_buf_get_name(0) },
        root_dir = vim.fs.root(0, { ".git", "pyproject.toml", "setup.py", "Cargo.toml", "go.mod" }),
        settings = {
          languages = {
            toml = {
              linters = {
                {
                  program = "tomllint",
                  args = { "-" },
                  pattern = "(.*):(\\d+):(\\d+): error: (.*)",
                  filename_match = 1,
                  line_match = 2,
                  start_col_match = 3,
                  description_match = 4,
                  use_stdin = true,
                  use_stderr = true,
                },
              },
            },
            sh = {
              linters = {
                {
                  program = "shellcheck",
                  args = {
                    "-f",
                    "gcc",
                    "-",
                  },
                  pattern = "(.*):(\\d+):(\\d+): (\\w+): (.*)",
                  filename_match = 1,
                  line_match = 2,
                  start_col_match = 3,
                  severity_match = 4,
                  description_match = 5,
                  use_stdin = true,
                  use_stderr = false,
                },
              },
            },
            python = {
              linters = {
                {
                  program = "mypy",
                  args = {
                    "--show-column-numbers",
                    "--show-error-end",
                    "--hide-error-codes",
                    "--hide-error-context",
                    "--no-color-output",
                    "--no-error-summary",
                    "--no-pretty",
                    "--shadow-file",
                    "$filename",
                    "/dev/stdin",
                    "$filename",
                  },
                  pattern = "(.*):(\\d+):(\\d+):\\d+:(\\d+): error: (.*)",
                  filename_match = 1,
                  line_match = 2,
                  start_col_match = 3,
                  end_col_match = 4,
                  description_match = 5,
                  use_stdin = true,
                  use_stderr = false,
                },
                {
                  program = "ruff",
                  args = {
                    "check",
                    "--stdin-filename",
                    "$filename",
                  },
                  pattern = "(.*):(\\d+):(\\d+): (.*)",
                  filename_match = 1,
                  line_match = 2,
                  start_col_match = 3,
                  description_match = 4,
                  use_stdin = true,
                  use_stderr = false,
                },
              },
            },
          },
        },
      }, {
        bufnr = 0,
        reuse_client = function(_, _)
          return false
        end,
      })
    else
      vim.notify(
        "unable to find 'lintls' executable. not registering lintls as a language server. " ..
        "see lintls-debug-runner for further instructions")
    end
  end,
})
```
