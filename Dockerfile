####################################################################################################
## Builder
####################################################################################################
FROM rust:latest AS builder

RUN update-ca-certificates
RUN apt-get update && apt-get -y install cmake
RUN apt-get -y install libssl-dev pkg-config
# Create appuser
ENV USER=tasker
ENV UID=10001

RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    "${USER}"


WORKDIR /tasker

COPY ./ .
RUN dir -s    

# We no longer need to use the x86_64-unknown-linux-musl target
RUN cargo build --release

####################################################################################################
## Final image
####################################################################################################
FROM debian:bullseye

# Install ca-certificates so that HTTPS works consistently
RUN apt-get update && apt-get install -y ca-certificates

# Import from builder.
COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /etc/group /etc/group

WORKDIR /tasker

# Copy our build
COPY --from=builder /tasker/target/release/task-worker-rust ./
RUN ls -a

# Use an unprivileged user.
USER tasker:tasker

ENTRYPOINT ["./task-worker-rust"]
