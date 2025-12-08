// UCache 性能测试
#include <stdio.h>
#include <fcntl.h>
#include <unistd.h>
#include <time.h>

#define BUF_SIZE 4096
#define FILE_SIZE (1024 * 1024)  // 1MB

// 测试1：顺序读取性能
void test_sequential_read() {
    printf("=== Test 1: Sequential Read ===\n");
    
    int fd = open("/testfile", O_RDONLY, 0);
    if (fd < 0) {
        printf("Failed to open file\n");
        return;
    }

    char buf[BUF_SIZE];
    clock_t start = clock();
    
    ssize_t total = 0;
    ssize_t n;
    while ((n = read(fd, buf, BUF_SIZE)) > 0) {
        total += n;
    }
    
    clock_t end = clock();
    double time_ms = (double)(end - start) * 1000 / CLOCKS_PER_SEC;
    
    printf("Read %ld bytes in %.2f ms\n", total, time_ms);
    printf("Throughput: %.2f MB/s\n", (total / (1024.0 * 1024.0)) / (time_ms / 1000.0));
    
    close(fd);
}

// 测试2：随机读取性能
void test_random_read() {
    printf("\n=== Test 2: Random Read ===\n");
    
    int fd = open("/testfile", O_RDONLY, 0);
    if (fd < 0) return;

    char buf[BUF_SIZE];
    clock_t start = clock();
    
    // 随机跳跃读取
    for (int i = 0; i < 100; i++) {
        off_t offset = (i * 7919) % FILE_SIZE;  // 伪随机偏移
        lseek(fd, offset, SEEK_SET);
        read(fd, buf, BUF_SIZE);
    }
    
    clock_t end = clock();
    printf("Random read 100 pages in %.2f ms\n", 
           (double)(end - start) * 1000 / CLOCKS_PER_SEC);
    
    close(fd);
}

int main() {
    printf("UCache Performance Test\n");
    printf("Expected: Cache hit rate > 80%% for sequential reads\n\n");
    
    test_sequential_read();
    test_random_read();
    
    return 0;
}
