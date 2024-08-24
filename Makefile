repository:
	docker compose up -d
repository-down:
	docker compose down
fetch:
	docker run --rm \
		-e REPO_BASE_URL="http://repository:8080" \
		-e FUND_BASE_URL="https://api.fund.eastmoney.com/f10/lsjz" \
		-e COOKIE_PATH="/app/cookie" \
		--name fundation-fetcher \
		--network fundation_default \
		fundation/fetcher:v0.1


# deb https://mirrors.tuna.tsinghua.edu.cn/ubuntu/ noble main restricted universe multiverse
# # deb-src https://mirrors.tuna.tsinghua.edu.cn/ubuntu/ noble main restricted universe multiverse
# deb https://mirrors.tuna.tsinghua.edu.cn/ubuntu/ noble-updates main restricted universe multiverse
# # deb-src https://mirrors.tuna.tsinghua.edu.cn/ubuntu/ noble-updates main restricted universe multiverse
# deb https://mirrors.tuna.tsinghua.edu.cn/ubuntu/ noble-backports main restricted universe multiverse
# # deb-src https://mirrors.tuna.tsinghua.edu.cn/ubuntu/ noble-backports main restricted universe multiverse