# rs-diff

## 功能
用于对比两个API的Response的不同

## 运行
```bash
cargo build

./target/debug/xdiff run -p todo -c  ./fixtures/test.yml -e a=10 -e @b=2 -e %c=3 -e m=10 
```