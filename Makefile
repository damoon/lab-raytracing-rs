all: test clock.png projectile.png scene-1.png scene-2.png scene-3.png shadow.png sphere-normals.png sphere-shading.png sphere-silhouette.png planes.png patterns.png reflections.png refraction.png

clean:
	rm -f *.png *.ppm perf.* profile* flamegraph*.svg

#flamegraph-cargo.svg:
#	echo -1 | sudo tee /proc/sys/kernel/perf_event_paranoid
#	echo 0 | sudo tee /proc/sys/kernel/kptr_restrict
#	cargo flamegraph --output flamegraph-cargo.svg --example refraction | convert /dev/stdin /dev/null

profile:
	PROFILE_CPU=1 cargo run --release --example refraction | convert /dev/stdin refraction.png
	pprof -output profile.svg -svg target/release/examples/refraction profile.pb

test:
	cargo test --test cucumber -- --silent

clock.png:
	cargo run --release --example clock | convert /dev/stdin clock.png

projectile.png:
	cargo run --release --example projectile-ppm | convert /dev/stdin projectile.png

scene-1.png:
	cargo run --release --example scene 1 | convert /dev/stdin scene-1.png
scene-2.png:
	cargo run --release --example scene 2 | convert /dev/stdin scene-2.png
scene-3.png:
	cargo run --release --example scene 3 | convert /dev/stdin scene-3.png

shadow.png:
	cargo run --release --example shadow | convert /dev/stdin shadow.png

sphere-normals.png:
	cargo run --release --example sphere-normals | convert /dev/stdin sphere-normals.png
sphere-shading.png:
	cargo run --release --example sphere-shading | convert /dev/stdin sphere-shading.png
sphere-silhouette.png:
	cargo run --release --example sphere-silhouette | convert /dev/stdin sphere-silhouette.png

planes.png:
	cargo run --release --example planes | convert /dev/stdin planes.png

patterns.png:
	cargo run --release --example patterns | convert /dev/stdin patterns.png

reflections.png:
	cargo run --release --example reflections | convert /dev/stdin reflections.png

refraction.png:
	cargo run --release --example refraction | convert /dev/stdin refraction.png
