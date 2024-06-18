Docker:
	docker build -f .\Dockerfile -t rustapi:1.0 .
	docker images | grep 'rustapi'