%define __spec_install_post %{nil}
%define __os_install_post %{_dbpath}/brp-compress
%define debug_package %{nil}
%define projectpath ../../../../../..

Name: etopa
Summary: Etopa HTTPS API
Version: @@VERSION@@
Release: @@RELEASE@@%{?dist}
License: ISC
Group: System Environment/Daemons
Source0: %{name}-%{version}.tar.gz
URL: https://etopa.de

BuildRoot: %{_tmppath}/%{name}-%{version}-%{release}-root
BuildRequires: systemd

Requires(post): systemd
Requires(preun): systemd
Requires(postun): systemd

%description
%{summary}

%prep
%setup

%install
rm -rf %{buildroot}
mkdir -p %{buildroot}
cp -a * %{buildroot}
mkdir -p %{buildroot}/usr/share/doc/etopa/
cp %{projectpath}/NOTICE.txt %{buildroot}/usr/share/doc/etopa/
mkdir -p %{buildroot}/var/lib/etopa/data/
cp %{projectpath}/data/*.pem %{buildroot}/var/lib/etopa/data/
mkdir -p %{buildroot}/etc/
cp %{projectpath}/etopai/.rpm/etopa.conf %{buildroot}/etc/

%clean
rm -rf %{buildroot}

%pre
if ! id -u etopa > /dev/null 2>&1; then
  useradd -M -s /bin/false etopa
fi

%post
chown -R etopa:etopa /var/lib/etopa
%systemd_post etopa.service
systemctl enable etopa
systemctl start etopa

%preun
%systemd_preun etopa.service
systemctl stop etopa
systemctl disable etopa

%postun
%systemd_postun_with_restart etopa.service

%files
%defattr(-,root,root,-)
%{_bindir}/*
%{_unitdir}/etopa.service
%defattr(644,etopa,etopa)
%dir /var/lib/etopa
/usr/share/doc/etopa/NOTICE.txt
%config(noreplace)
/etc/etopa.conf
/var/lib/etopa/data/*
