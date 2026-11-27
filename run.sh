#!/usr/bin/env bash
set -u

BINARY="./target/release/lebrel-bot"
LOG_FILE="lebrel-bot.log"
SESSION_NAME="lebrel-bot"

build_release() {
    echo "Building release binary..."
    if ! cargo build --release; then
        echo "Build failed. Fix the errors above and run ./run.sh build again."
        exit 1
    fi
}

run_in_fg() {
    if [ ! -x "$BINARY" ]; then
        build_release
    fi
    "$BINARY" run
}

start_bg() {
    if ! command -v screen >/dev/null 2>&1; then
        echo "screen is not installed. Install it or use ./run.sh run instead."
        exit 1
    fi
    if screen -list 2>/dev/null | grep -q "\.${SESSION_NAME}[[:space:]]"; then
        echo "Session ${SESSION_NAME} is already running. Attach with screen -r ${SESSION_NAME}."
        exit 0
    fi
    build_release
    screen -dmS "$SESSION_NAME" bash -c "$BINARY run 2>&1 | tee -a $LOG_FILE"
    sleep 2
    echo "Started in background. Attach with screen -r ${SESSION_NAME}. Log: ${LOG_FILE}"
}

stop_bg() {
    if screen -list 2>/dev/null | grep -q "\.${SESSION_NAME}[[:space:]]"; then
        screen -S "${SESSION_NAME}.${SESSION_NAME}" -X quit 2>/dev/null || screen -S "$SESSION_NAME" -X quit
        echo "Stopped ${SESSION_NAME}."
    else
        echo "No running session named ${SESSION_NAME}."
    fi
}

case "${1:-run}" in
    build)
        build_release
        ;;
    run)
        run_in_fg
        ;;
    probe)
        if [ ! -x "$BINARY" ]; then
            build_release
        fi
        "$BINARY" probe
        ;;
    selftest)
        if [ ! -x "$BINARY" ]; then
            build_release
        fi
        "$BINARY" selftest
        ;;
    create)
        if [ ! -x "$BINARY" ]; then
            build_release
        fi
        shift
        "$BINARY" create "${1:-1}"
        ;;
    bg)
        start_bg
        ;;
    bgprobe)
        if [ ! -x "$BINARY" ]; then
            build_release
        fi
        screen -dmS "${SESSION_NAME}-probe" bash -c "$BINARY probe 2>&1 | tee -a $LOG_FILE"
        echo "Probe started in background."
        ;;
    screen)
        if screen -list 2>/dev/null | grep -q "\.${SESSION_NAME}[[:space:]]"; then
            screen -r "$SESSION_NAME"
        else
            echo "No running session named ${SESSION_NAME}."
        fi
        ;;
    stop)
        stop_bg
        ;;
    *)
        echo "Usage: ./run.sh [build|run|probe|selftest|create <count>|bg|bgprobe|screen|stop]"
        exit 1
        ;;
esac