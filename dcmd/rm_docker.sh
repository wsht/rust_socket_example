docker ps -a | grep -vi "container" | awk '{print $1}' | xargs docker rm