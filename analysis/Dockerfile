FROM rocker/verse:3.5.3

USER rstudio
WORKDIR /analysis

ENTRYPOINT ["/usr/local/bin/Rscript", "--vanilla"]

CMD ["render.R"]
