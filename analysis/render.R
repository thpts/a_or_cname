library(rmarkdown)

dataDir <- '/data/'

render('index.Rmd', c("html_document", "pdf_document"))
