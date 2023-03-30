#! /bin/bash
set -e # Enable errexit

dirname=~/Projects/hipages/phinx-migrations/
pname=${1:SeedProfileStatsBatch}
echo "use phinx name: "$pname
cd $dirname
git checkout master
git pull origin master
git checkout -B $pname
git status
php vendor/bin/phinx create $pname --configuration hip/phinx.yml
fileName=$(git status --short | awk '{print $2}')
cd -
echo "copy migration_output.php to $dirname$fileName"
cat migration_output.php >$dirname$fileName

# check then raise PR
cd $dirname
git add . && git commit -m 'feat: seed profile stats batch'
git push origin $pname
# after merge
# ./k8s_phinx.sh hip migrate production
