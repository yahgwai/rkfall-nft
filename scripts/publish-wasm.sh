rm -rf ./web/pkg
wasm-pack build --out-dir ./web/pkg --target web
timestamp=$(date +%s)
find ./web/pkg/rk_fall.js -type f -exec sed -i -E 's/rk_fall_bg/rk_fall_bg\.'${timestamp}'/g' {} \;
mv ./web/pkg/rk_fall_bg.wasm ./web/pkg/rk_fall_bg.${timestamp}.wasm
find ./web/view.html -type f -exec sed -i -E 's/rk_fall\.[0-9]+/rk_fall\.'${timestamp}'/g' {} \;
mv ./web/pkg/rk_fall.js ./web/pkg/rk_fall.${timestamp}.js