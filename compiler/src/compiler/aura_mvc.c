#ifndef WIN32_LEAN_AND_MEAN
#define WIN32_LEAN_AND_MEAN
#endif

#ifdef _WIN32
    #include <WinSock2.h>
    #include <Windows.h>
    #include <WS2tcpip.h>
    #include <stdio.h>
    #include <stdlib.h>
    #include <string.h>
#else
    #include <sys/socket.h>
    #include <netinet/in.h>
    #include <unistd.h>
    #include <stdio.h>
    #include <stdlib.h>
    #include <string.h>
#endif

// Aura'nın beklediği fonksiyon imzası
typedef char* (*AuraControllerFunc)(void* instance, int param);

typedef struct {
    char path[64];
    AuraControllerFunc func;
} Route;

static Route registry[32];
static int route_count = 0;

void aura_mvc_register(const char* path, AuraControllerFunc func_ptr) {
    if (route_count < 32) {
        // strncpy yerine güvenli kopyalama
        size_t i = 0;
        for (i = 0; i < 63 && path[i] != '\0'; i++) {
            registry[route_count].path[i] = path[i];
        }
        registry[route_count].path[i] = '\0';
        registry[route_count].func = func_ptr;
        route_count++;
    }
}

void aura_mvc_serve(int sock, void* instance) {
    if (sock == -1) return;

    printf("MVC Server listening...\n");
    while (1) {
#ifdef _WIN32
        int client_sock = (int)accept(sock, NULL, NULL);
#else
        int client_sock = accept(sock, NULL, NULL);
#endif
        if (client_sock == -1) continue;

        char buffer[1024] = {0};
        recv(client_sock, buffer, 1024, 0);

        // Basit routing
        char* path_start = strstr(buffer, "GET /");
        if (path_start) {
            path_start += 5;
            char* path_end = strpbrk(path_start, " \r\n");
            if (path_end) {
                char method_name[64] = {0};
                size_t len = path_end - path_start;
                char* q_mark = strchr(path_start, '?');
                if (q_mark && q_mark < path_end) {
                    len = q_mark - path_start;
                }
                if (len > 63) len = 63;
                
                // Safe copy
                for (size_t i = 0; i < len; i++) {
                    method_name[i] = path_start[i];
                }
                method_name[len] = '\0';

                // Registry'de ara
                int found = 0;
                for (int i = 0; i < route_count; i++) {
                    if (strcmp(registry[i].path, method_name) == 0) {
                        // Parametre parse (basitçe ?'dan sonraki = sonrasını alıyoruz)
                        int param = 0;
                        char* q = strchr(path_start, '?');
                        if (q && q < path_end) {
                            char* eq = strchr(q, '=');
                            if (eq && eq < path_end) {
                                param = atoi(eq + 1);
                            }
                        }

                        char* response_body = registry[i].func(instance, param);
                        
                        char header[256];
                        int h_len = sprintf(header, "HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=utf-8\r\nContent-Length: %zu\r\n\r\n", strlen(response_body));
                        
                        send(client_sock, header, h_len, 0);
                        send(client_sock, response_body, (int)strlen(response_body), 0);
                        found = 1;
                        break;
                    }
                }

                if (!found) {
                    char error404[] = "HTTP/1.1 404 Not Found\r\n\r\n{\"error\":\"Route not found\"}";
                    send(client_sock, error404, (int)strlen(error404), 0);
                }
            }
        }

#ifdef _WIN32
        closesocket(client_sock);
#else
        close(client_sock);
#endif
    }
}
