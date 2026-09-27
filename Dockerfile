FROM gcr.io/distroless/cc-debian13

COPY --chmod=755 dist/popineau_eu /popineau_eu
COPY dist/site /site

ENV LEPTOS_OUTPUT_NAME=popineau_eu
ENV LEPTOS_SITE_ROOT=/site
ENV LEPTOS_SITE_PKG_DIR=pkg
ENV LEPTOS_SITE_ADDR=0.0.0.0:8080

ENTRYPOINT ["/popineau_eu"]
CMD []
