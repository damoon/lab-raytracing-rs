all: test clock.ppm projectile.ppm scene-1.ppm scene-2.ppm scene-3.ppm shadow.ppm sphere-normals.ppm sphere-shading.ppm sphere-silhouette.ppm

test:
	cargo test --test cucumber -- --silent

clock.ppm:
	cargo run --release --example clock > clock.ppm

projectile.ppm:
	cargo run --release --example projectile-ppm > projectile.ppm

scene-1.ppm:
	cargo run --release --example scene 1 > scene-1.ppm
scene-2.ppm:
	cargo run --release --example scene 2 > scene-2.ppm
scene-3.ppm:
	cargo run --release --example scene 3 > scene-3.ppm

shadow.ppm:
	cargo run --release --example shadow > shadow.ppm

sphere-normals.ppm:
	cargo run --release --example sphere-normals > sphere-normals.ppm
sphere-shading.ppm:
	cargo run --release --example sphere-shading > sphere-shading.ppm
sphere-silhouette.ppm:
	cargo run --release --example sphere-silhouette > sphere-silhouette.ppm
