from sphinx import __display_version__

extensions = [
  "sphinx.ext.todo"
]

project = "STARBLOOM"
copyright = "2026-%Y John Schiefelebein"
release = version = __display_version__
show_authors = True

html_theme = "scrolls"
