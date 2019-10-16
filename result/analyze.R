library(ggplot2)
library(ggsci)

input_fname <- "result.txt"

x <- read.table(input_fname, header = TRUE)
x["ratio"] <- x["c"] / x["rust"]
benchmark <- c("matmult_int", "nbody", "st", "aha_mont64", "crc32", "minver", "cubic", "nettle_aes")

g <- ggplot(x, aes(x = benchmark, y = ratio, fill = benchmark))
g <- g + geom_bar(stat = "identity")
g <- g + scale_fill_nejm()
g <- g + xlab("")
g <- g + theme(axis.text.x=element_blank())
g <- g + theme(text = element_text(size = 20, colour="black"))
plot(g)
ggsave("result.eps")