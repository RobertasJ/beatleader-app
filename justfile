hotreload:
   dx serve --package beatleader-app  --hot-patch --features hotreload

devtools:
   dx serve --package beatleader-app  --hot-patch --features=hotreload,devtools

hotreload-backtrace:
   RUST_BACKTRACE=1 dx serve --package beatleader-app --hot-patch --features hotreload