use once_cell::sync::Lazy;
use users::{get_user_by_uid, get_current_uid};

pub static CURR_USER: Lazy<users::User> = Lazy::new(|| {
    get_user_by_uid(get_current_uid()).unwrap()
});

/// A list of all the common logging files in a UNIX machine
pub static LOG_FILES: [&str; 60] = [
    // shell history
    "~/.bash_history",
    "~/.zsh_history",

    // system logs
    "/var/log/syslog",
    "/var/log/messages",
    "/var/log/auth.log",
    "/var/log/secure",
    "/var/log/kern.log",
    "/var/log/daemon.log",
    "/var/log/cron.log",
    "/var/log/boot.log",
    "/var/log/dmesg",

    // login records (binary)
    "/var/log/wtmp",
    "/var/log/btmp",
    "/var/log/lastlog",
    "/var/log/faillog",

    // package managers / installers
    "/var/log/dpkg.log",
    "/var/log/apt/history.log",
    "/var/log/yum.log",

    // systemd / journal
    "/var/log/journal",             // systemd journal DB directory
    "/run/log/journal",             // alternate journal path

    // audit
    "/var/log/audit/audit.log",

    // webserver / app logs
    "/var/log/nginx/access.log",
    "/var/log/nginx/error.log",
    "/var/log/apache2/access.log",
    "/var/log/apache2/error.log",
    "/var/log/httpd/access_log",
    "/var/log/httpd/error_log",

    // mail / mailagents
    "/var/log/mail.log",
    "/var/log/maillog",

    // database / cache / search
    "/var/log/mysql/error.log",
    "/var/log/mysqld.log",
    "/var/log/postgresql/postgresql.log",
    "/var/log/mongodb/mongod.log",
    "/var/log/redis/redis-server.log",
    "/var/log/elasticsearch/elasticsearch.log",

    // common daemon/service logs
    "/var/log/supervisor/supervisord.log",
    "/var/log/cron",                // some distros use dir
    "/var/log/pm-powersave.log",

    // Docker / container runtimes
    "/var/log/docker.log",                                  // docker daemon (old/hypothetical)
    "/var/lib/docker/containers/*/*-json.log",              // per-container Docker logs (json-file driver)
    "/var/log/containers/*",                                // k8s-style symlinks to container logs
    "/var/log/pods/*",                                      // k8s pods logs directory
    "/var/log/daemon.log",                                  // may include container runtime events
    "/var/log/containerd.log",                              // containerd daemon log
    "/run/containerd",                                      // containerd runtime artifacts (may contain logs)

    // CRI-O
    "/var/log/crio/crio.log",
    "/var/log/crio",                                        // CRI-O logs dir

    // Kubernetes node & control-plane logs
    "/var/log/kubelet.log",
    "/var/log/kube-proxy.log",
    "/var/log/kube-apiserver.log",
    "/var/log/kube-scheduler.log",
    "/var/log/kube-controller-manager.log",
    "/var/log/etcd/etcd.log",
    "/var/log/etcd",                                        // etcd data/log dir (distro dependent)
    "/var/log/kubernetes",                                  // some installs aggregate here

    // monitoring / observability
    "/var/log/prometheus/*.log",
    "/var/log/grafana/grafana.log",
    "/var/log/node-exporter.log",
    "/var/log/collectd.log",

    // app/platform specific & misc
    "/var/log/php7.4-fpm.log",
    "/var/log/php-fpm.log",
    "/var/log/solr/solr.log",
    "/var/log/cassandra/system.log",
    "/var/log/zookeeper/zookeeper.log",
    "/var/log/uwsgi/app/*.log",
    "/var/log/krb5kdc.log",

    // container runtime logs that can be symlinked under /var/log/containers/
    "/var/log/pods/*/*/*.log",

    // user/application directories that often contain logs (project-local)
    "/var/log/*",                                           // wildcard: review carefully
    "/var/lib/docker/volumes/*/_data/*",                    // common place for dockerized app logs
]; 
// Thanks https://github.com/sundowndev/covermyass
// Added logs by havij13
