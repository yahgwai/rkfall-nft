rm -rf ./web/pkg
wasm-pack build --out-dir ./web/pkg --target web
timestamp=$(date +%s)
find ./web/pkg/three_body.js -type f -exec sed -i -E 's/three_body_bg/three_body_bg\.'${timestamp}'/g' {} \;
mv ./web/pkg/three_body_bg.wasm ./web/pkg/three_body_bg.${timestamp}.wasm
find ./web/view.html -type f -exec sed -i -E 's/three_body\.[0-9]+/three_body\.'${timestamp}'/g' {} \;
mv ./web/pkg/three_body.js ./web/pkg/three_body.${timestamp}.js