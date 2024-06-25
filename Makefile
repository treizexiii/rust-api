Docker:
	docker build -f .\Dockerfile -t rustapi:1.0 .
	docker images | grep 'rustapi'

Run:
	docker run -p 8000:8000 rustapi:1.0
	docker ps | grep 'rustapi'
