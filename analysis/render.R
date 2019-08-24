library(rmarkdown)

dataDir <- '/data/'

render('journal-cyber-policy-2019/index.Rmd', c("pdf_document", "html_document"))
