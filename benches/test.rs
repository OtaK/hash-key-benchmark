use hash_bound_benchmark::ByteMap;

fn benchmark(c: &mut criterion::Criterion) {
    let bytemap = ByteMap::new();
    let mut hashmap = std::collections::HashMap::new();
    hashmap.insert(b"key1", b"value1");
    hashmap.insert(b"key2", b"value2");
    hashmap.insert(b"key3", b"value3");
    hashmap.insert(b"key4", b"value4");
    hashmap.insert(b"key5", b"value5");

    c.bench_function("bytemap_read", |b| b.iter(|| criterion::black_box(&bytemap[b"key4"])));
    c.bench_function("hashmap_read", |b| b.iter(|| criterion::black_box(hashmap[b"key4"])));
}

criterion::criterion_group!(benches, benchmark);
criterion::criterion_main!(benches);
