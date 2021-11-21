all: renders clock.png projectile.png

renders: scene-1.png scene-2.png scene-3.png shadow.png sphere-normals.png sphere-shading.png \
	sphere-silhouette.png planes.png patterns.png reflections.png refraction.png cubes.png \
	cylinders-cones.png groups.png many-spheres.png metallic.png hexagon-donut.png dodecahedron.png \
	teapot.png teapot-low.png teapot-high.png dragon.png astronaut.png csg.png

clean:
	rm -f *.png *.ppm perf.* profile* flamegraph*.svg

#flamegraph-cargo.svg:
#	echo -1 | sudo tee /proc/sys/kernel/perf_event_paranoid
#	echo 0 | sudo tee /proc/sys/kernel/kptr_restrict
#	cargo flamegraph --output flamegraph-cargo.svg --example refraction | convert /dev/stdin /dev/null

export EXAMPLE=teapot

profile:
	PROFILE_CPU=1 cargo run --release --example $(EXAMPLE) | convert /dev/stdin $(EXAMPLE).png
	pprof -output profile.svg -svg target/release/examples/$(EXAMPLE) profile.pb
	time target/release/examples/$(EXAMPLE) > /dev/null

test:
	cargo test --test cucumber

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

cubes.png:
	cargo run --release --example cubes | convert /dev/stdin cubes.png

cylinders-cones.png:
	cargo run --release --example cylinders-cones | convert /dev/stdin cylinders-cones.png

groups.png:
	cargo run --release --example groups | convert /dev/stdin groups.png

many-spheres.png:
	cargo run --release --example many-spheres | convert /dev/stdin many-spheres.png

metallic.png:
	cargo run --release --example metallic | convert /dev/stdin metallic.png

hexagon-donut.png:
	cargo run --release --example hexagon-donut | convert /dev/stdin hexagon-donut.png

dodecahedron.png:
	cargo run --release --example dodecahedron | convert /dev/stdin dodecahedron.png

teapot.png:
	cargo run --release --example obj_file y examples/teapot.obj | convert /dev/stdin teapot.png

teapot-low.png:
	cargo run --release --example obj_file z examples/teapot-low.obj | convert /dev/stdin teapot-low.png

teapot-high.png:
	cargo run --release --example obj_file z examples/teapot-high.obj | convert /dev/stdin teapot-high.png

dragon.png:
	cargo run --release --example obj_file y examples/dragon.obj | convert /dev/stdin dragon.png

astronaut.png:
	cargo run --release --example obj_file y examples/astronaut.obj | convert /dev/stdin astronaut.png

csg.png:
	cargo run --release --example csg | convert /dev/stdin csg.png
