FROM gcr.io/distroless/cc-debian13

COPY --chmod=755 dist/popineau_eu /popineau_eu
COPY dist/assets /assets

ENTRYPOINT ["/popineau_eu"]
CMD []
