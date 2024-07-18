PACKAGES=$(grep "'" Cargo.toml | grep -v '#' | tr -d "'" | tr -d ",")

for package in $PACKAGES
do
    git clone --depth=1 git@github.com:ancos2505/${package}.git

done

