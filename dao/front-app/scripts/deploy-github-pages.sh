rm -rf ./dist
yarn build
cd ./dist
# echo "---\npermalink: /index.html\n---" > ./404.html
cp ./index.html ./404.html
git init
git add -A
git commit -m "first commit"
git branch -M main
git remote add origin git@github.com:fabiooshiro/fabiooshiro.github.io.git
git push -u origin main -f
