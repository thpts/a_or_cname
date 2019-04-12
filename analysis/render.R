library(rmarkdown)

dataDir <- '/data/'

render('ietf-dnsop-2018-11-21/index.Rmd', c("html_document", "pdf_document"))
