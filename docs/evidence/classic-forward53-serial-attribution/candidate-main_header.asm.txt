Disassembly of section .text:

00000000000d74b0 <emuella_j2k_codestream::write_native_main_header>:
   d74b0:	55                                              	push   %rbp
   d74b1:	41 57                                           	push   %r15
   d74b3:	41 56                                           	push   %r14
   d74b5:	41 55                                           	push   %r13
   d74b7:	41 54                                           	push   %r12
   d74b9:	53                                              	push   %rbx
   d74ba:	48 83 ec 28                                     	sub    $0x28,%rsp
   d74be:	44 89 4c 24 24                                  	mov    %r9d,0x24(%rsp)
   d74c3:	44 89 44 24 20                                  	mov    %r8d,0x20(%rsp)
   d74c8:	89 4c 24 1c                                     	mov    %ecx,0x1c(%rsp)
   d74cc:	89 54 24 18                                     	mov    %edx,0x18(%rsp)
   d74d0:	49 89 f4                                        	mov    %rsi,%r12
   d74d3:	48 89 7c 24 10                                  	mov    %rdi,0x10(%rsp)
   d74d8:	0f b7 5c 24 68                                  	movzwl 0x68(%rsp),%ebx
   d74dd:	0f b6 84 24 88 00 00 00                         	movzbl 0x88(%rsp),%eax
   d74e5:	88 44 24 07                                     	mov    %al,0x7(%rsp)
   d74e9:	48 8b ac 24 80 00 00 00                         	mov    0x80(%rsp),%rbp
   d74f1:	48 8b 44 24 78                                  	mov    0x78(%rsp),%rax
   d74f6:	48 89 44 24 08                                  	mov    %rax,0x8(%rsp)
   d74fb:	0f b6 44 24 70                                  	movzbl 0x70(%rsp),%eax
   d7500:	88 44 24 06                                     	mov    %al,0x6(%rsp)
   d7504:	44 0f b6 7c 24 60                               	movzbl 0x60(%rsp),%r15d
   d750a:	85 db                                           	test   %ebx,%ebx
   d750c:	74 33                                           	je     d7541 <emuella_j2k_codestream::write_native_main_header+0x91>
   d750e:	44 8d 2c 9d 00 00 00 00                         	lea    0x0(,%rbx,4),%r13d
   d7516:	4c 89 ef                                        	mov    %r13,%rdi
   d7519:	ff 15 99 d8 19 00                               	call   *0x19d899(%rip)        # 274db8 <malloc@GLIBC_2.2.5>
   d751f:	48 85 c0                                        	test   %rax,%rax
   d7522:	0f 84 4a 09 00 00                               	je     d7e72 <emuella_j2k_codestream::write_native_main_header+0x9c2>
   d7528:	49 89 c6                                        	mov    %rax,%r14
   d752b:	41 0f b6 c7                                     	movzbl %r15b,%eax
   d752f:	c1 e0 08                                        	shl    $0x8,%eax
   d7532:	0d 00 00 01 01                                  	or     $0x1010000,%eax
   d7537:	66 83 fb 08                                     	cmp    $0x8,%bx
   d753b:	73 0c                                           	jae    d7549 <emuella_j2k_codestream::write_native_main_header+0x99>
   d753d:	31 c9                                           	xor    %ecx,%ecx
   d753f:	eb 4f                                           	jmp    d7590 <emuella_j2k_codestream::write_native_main_header+0xe0>
   d7541:	41 be 01 00 00 00                               	mov    $0x1,%r14d
   d7547:	eb 53                                           	jmp    d759c <emuella_j2k_codestream::write_native_main_header+0xec>
   d7549:	89 d9                                           	mov    %ebx,%ecx
   d754b:	83 e1 f8                                        	and    $0xfffffff8,%ecx
   d754e:	66 0f 6e c0                                     	movd   %eax,%xmm0
   d7552:	66 0f 70 c0 00                                  	pshufd $0x0,%xmm0,%xmm0
   d7557:	8d 14 9d 00 00 00 00                            	lea    0x0(,%rbx,4),%edx
   d755e:	83 e2 e0                                        	and    $0xffffffe0,%edx
   d7561:	31 f6                                           	xor    %esi,%esi
   d7563:	66 66 66 66 2e 0f 1f 84 00 00 00 00 00          	data16 data16 data16 cs nopw 0x0(%rax,%rax,1)
   d7570:	f3 41 0f 7f 04 36                               	movdqu %xmm0,(%r14,%rsi,1)
   d7576:	f3 41 0f 7f 44 36 10                            	movdqu %xmm0,0x10(%r14,%rsi,1)
   d757d:	48 83 c6 20                                     	add    $0x20,%rsi
   d7581:	48 39 f2                                        	cmp    %rsi,%rdx
   d7584:	75 ea                                           	jne    d7570 <emuella_j2k_codestream::write_native_main_header+0xc0>
   d7586:	39 d9                                           	cmp    %ebx,%ecx
   d7588:	74 12                                           	je     d759c <emuella_j2k_codestream::write_native_main_header+0xec>
   d758a:	66 0f 1f 44 00 00                               	nopw   0x0(%rax,%rax,1)
   d7590:	41 89 04 8e                                     	mov    %eax,(%r14,%rcx,4)
   d7594:	48 ff c1                                        	inc    %rcx
   d7597:	48 39 cb                                        	cmp    %rcx,%rbx
   d759a:	75 f4                                           	jne    d7590 <emuella_j2k_codestream::write_native_main_header+0xe0>
   d759c:	48 83 fd 07                                     	cmp    $0x7,%rbp
   d75a0:	0f 85 8a 02 00 00                               	jne    d7830 <emuella_j2k_codestream::write_native_main_header+0x380>
   d75a6:	66 b9 03 00                                     	mov    $0x3,%cx
   d75aa:	89 d8                                           	mov    %ebx,%eax
   d75ac:	66 f7 e1                                        	mul    %cx
   d75af:	0f 80 85 05 00 00                               	jo     d7b3a <emuella_j2k_codestream::write_native_main_header+0x68a>
   d75b5:	89 c5                                           	mov    %eax,%ebp
   d75b7:	66 83 f8 d9                                     	cmp    $0xffd9,%ax
   d75bb:	0f 87 9a 05 00 00                               	ja     d7b5b <emuella_j2k_codestream::write_native_main_header+0x6ab>
   d75c1:	49 8b 04 24                                     	mov    (%r12),%rax
   d75c5:	4d 8b 6c 24 10                                  	mov    0x10(%r12),%r13
   d75ca:	4c 29 e8                                        	sub    %r13,%rax
   d75cd:	48 83 f8 01                                     	cmp    $0x1,%rax
   d75d1:	0f 86 b7 05 00 00                               	jbe    d7b8e <emuella_j2k_codestream::write_native_main_header+0x6de>
   d75d7:	49 8b 44 24 08                                  	mov    0x8(%r12),%rax
   d75dc:	66 42 c7 04 28 ff 4f                            	movw   $0x4fff,(%rax,%r13,1)
   d75e3:	49 83 c5 02                                     	add    $0x2,%r13
   d75e7:	4d 89 6c 24 10                                  	mov    %r13,0x10(%r12)
   d75ec:	49 8b 04 24                                     	mov    (%r12),%rax
   d75f0:	4c 29 e8                                        	sub    %r13,%rax
   d75f3:	48 83 f8 01                                     	cmp    $0x1,%rax
   d75f7:	0f 86 b6 05 00 00                               	jbe    d7bb3 <emuella_j2k_codestream::write_native_main_header+0x703>
   d75fd:	49 8b 44 24 08                                  	mov    0x8(%r12),%rax
   d7602:	66 42 c7 04 28 ff 51                            	movw   $0x51ff,(%rax,%r13,1)
   d7609:	49 83 c5 02                                     	add    $0x2,%r13
   d760d:	4d 89 6c 24 10                                  	mov    %r13,0x10(%r12)
   d7612:	49 8b 04 24                                     	mov    (%r12),%rax
   d7616:	4c 29 e8                                        	sub    %r13,%rax
   d7619:	48 83 f8 01                                     	cmp    $0x1,%rax
   d761d:	0f 86 b5 05 00 00                               	jbe    d7bd8 <emuella_j2k_codestream::write_native_main_header+0x728>
   d7623:	83 c5 26                                        	add    $0x26,%ebp
   d7626:	66 c1 c5 08                                     	rol    $0x8,%bp
   d762a:	49 8b 44 24 08                                  	mov    0x8(%r12),%rax
   d762f:	66 42 89 2c 28                                  	mov    %bp,(%rax,%r13,1)
   d7634:	49 83 c5 02                                     	add    $0x2,%r13
   d7638:	4d 89 6c 24 10                                  	mov    %r13,0x10(%r12)
   d763d:	49 8b 04 24                                     	mov    (%r12),%rax
   d7641:	4c 29 e8                                        	sub    %r13,%rax
   d7644:	48 83 f8 01                                     	cmp    $0x1,%rax
   d7648:	0f 86 af 05 00 00                               	jbe    d7bfd <emuella_j2k_codestream::write_native_main_header+0x74d>
   d764e:	49 8b 44 24 08                                  	mov    0x8(%r12),%rax
   d7653:	66 42 c7 04 28 00 00                            	movw   $0x0,(%rax,%r13,1)
   d765a:	49 83 c5 02                                     	add    $0x2,%r13
   d765e:	4d 89 6c 24 10                                  	mov    %r13,0x10(%r12)
   d7663:	49 8b 04 24                                     	mov    (%r12),%rax
   d7667:	4c 29 e8                                        	sub    %r13,%rax
   d766a:	48 83 f8 03                                     	cmp    $0x3,%rax
   d766e:	0f 86 ae 05 00 00                               	jbe    d7c22 <emuella_j2k_codestream::write_native_main_header+0x772>
   d7674:	8b 4c 24 18                                     	mov    0x18(%rsp),%ecx
   d7678:	0f c9                                           	bswap  %ecx
   d767a:	49 8b 44 24 08                                  	mov    0x8(%r12),%rax
   d767f:	42 89 0c 28                                     	mov    %ecx,(%rax,%r13,1)
   d7683:	49 83 c5 04                                     	add    $0x4,%r13
   d7687:	4d 89 6c 24 10                                  	mov    %r13,0x10(%r12)
   d768c:	49 8b 04 24                                     	mov    (%r12),%rax
   d7690:	4c 29 e8                                        	sub    %r13,%rax
   d7693:	48 83 f8 03                                     	cmp    $0x3,%rax
   d7697:	0f 86 aa 05 00 00                               	jbe    d7c47 <emuella_j2k_codestream::write_native_main_header+0x797>
   d769d:	8b 4c 24 1c                                     	mov    0x1c(%rsp),%ecx
   d76a1:	0f c9                                           	bswap  %ecx
   d76a3:	49 8b 44 24 08                                  	mov    0x8(%r12),%rax
   d76a8:	42 89 0c 28                                     	mov    %ecx,(%rax,%r13,1)
   d76ac:	49 83 c5 04                                     	add    $0x4,%r13
   d76b0:	4d 89 6c 24 10                                  	mov    %r13,0x10(%r12)
   d76b5:	49 8b 04 24                                     	mov    (%r12),%rax
   d76b9:	4c 29 e8                                        	sub    %r13,%rax
   d76bc:	48 83 f8 03                                     	cmp    $0x3,%rax
   d76c0:	0f 86 a6 05 00 00                               	jbe    d7c6c <emuella_j2k_codestream::write_native_main_header+0x7bc>
   d76c6:	49 8b 44 24 08                                  	mov    0x8(%r12),%rax
   d76cb:	42 c7 04 28 00 00 00 00                         	movl   $0x0,(%rax,%r13,1)
   d76d3:	49 83 c5 04                                     	add    $0x4,%r13
   d76d7:	4d 89 6c 24 10                                  	mov    %r13,0x10(%r12)
   d76dc:	49 8b 04 24                                     	mov    (%r12),%rax
   d76e0:	4c 29 e8                                        	sub    %r13,%rax
   d76e3:	48 83 f8 03                                     	cmp    $0x3,%rax
   d76e7:	0f 86 a4 05 00 00                               	jbe    d7c91 <emuella_j2k_codestream::write_native_main_header+0x7e1>
   d76ed:	49 8b 44 24 08                                  	mov    0x8(%r12),%rax
   d76f2:	42 c7 04 28 00 00 00 00                         	movl   $0x0,(%rax,%r13,1)
   d76fa:	49 83 c5 04                                     	add    $0x4,%r13
   d76fe:	4d 89 6c 24 10                                  	mov    %r13,0x10(%r12)
   d7703:	49 8b 04 24                                     	mov    (%r12),%rax
   d7707:	4c 29 e8                                        	sub    %r13,%rax
   d770a:	48 83 f8 03                                     	cmp    $0x3,%rax
   d770e:	0f 86 a2 05 00 00                               	jbe    d7cb6 <emuella_j2k_codestream::write_native_main_header+0x806>
   d7714:	8b 4c 24 20                                     	mov    0x20(%rsp),%ecx
   d7718:	0f c9                                           	bswap  %ecx
   d771a:	49 8b 44 24 08                                  	mov    0x8(%r12),%rax
   d771f:	42 89 0c 28                                     	mov    %ecx,(%rax,%r13,1)
   d7723:	49 83 c5 04                                     	add    $0x4,%r13
   d7727:	4d 89 6c 24 10                                  	mov    %r13,0x10(%r12)
   d772c:	49 8b 04 24                                     	mov    (%r12),%rax
   d7730:	4c 29 e8                                        	sub    %r13,%rax
   d7733:	48 83 f8 03                                     	cmp    $0x3,%rax
   d7737:	0f 86 9e 05 00 00                               	jbe    d7cdb <emuella_j2k_codestream::write_native_main_header+0x82b>
   d773d:	8b 4c 24 24                                     	mov    0x24(%rsp),%ecx
   d7741:	0f c9                                           	bswap  %ecx
   d7743:	49 8b 44 24 08                                  	mov    0x8(%r12),%rax
   d7748:	42 89 0c 28                                     	mov    %ecx,(%rax,%r13,1)
   d774c:	49 83 c5 04                                     	add    $0x4,%r13
   d7750:	4d 89 6c 24 10                                  	mov    %r13,0x10(%r12)
   d7755:	49 8b 04 24                                     	mov    (%r12),%rax
   d7759:	4c 29 e8                                        	sub    %r13,%rax
   d775c:	48 83 f8 03                                     	cmp    $0x3,%rax
   d7760:	0f 86 9a 05 00 00                               	jbe    d7d00 <emuella_j2k_codestream::write_native_main_header+0x850>
   d7766:	49 8b 44 24 08                                  	mov    0x8(%r12),%rax
   d776b:	42 c7 04 28 00 00 00 00                         	movl   $0x0,(%rax,%r13,1)
   d7773:	49 83 c5 04                                     	add    $0x4,%r13
   d7777:	4d 89 6c 24 10                                  	mov    %r13,0x10(%r12)
   d777c:	49 8b 04 24                                     	mov    (%r12),%rax
   d7780:	4c 29 e8                                        	sub    %r13,%rax
   d7783:	48 83 f8 03                                     	cmp    $0x3,%rax
   d7787:	0f 86 98 05 00 00                               	jbe    d7d25 <emuella_j2k_codestream::write_native_main_header+0x875>
   d778d:	49 8b 44 24 08                                  	mov    0x8(%r12),%rax
   d7792:	42 c7 04 28 00 00 00 00                         	movl   $0x0,(%rax,%r13,1)
   d779a:	49 83 c5 04                                     	add    $0x4,%r13
   d779e:	4d 89 6c 24 10                                  	mov    %r13,0x10(%r12)
   d77a3:	49 8b 04 24                                     	mov    (%r12),%rax
   d77a7:	4c 29 e8                                        	sub    %r13,%rax
   d77aa:	48 83 f8 01                                     	cmp    $0x1,%rax
   d77ae:	0f 86 96 05 00 00                               	jbe    d7d4a <emuella_j2k_codestream::write_native_main_header+0x89a>
   d77b4:	89 d8                                           	mov    %ebx,%eax
   d77b6:	66 c1 c0 08                                     	rol    $0x8,%ax
   d77ba:	49 8b 4c 24 08                                  	mov    0x8(%r12),%rcx
   d77bf:	66 42 89 04 29                                  	mov    %ax,(%rcx,%r13,1)
   d77c4:	49 83 c5 02                                     	add    $0x2,%r13
   d77c8:	4d 89 6c 24 10                                  	mov    %r13,0x10(%r12)
   d77cd:	45 84 ff                                        	test   %r15b,%r15b
   d77d0:	0f 84 25 03 00 00                               	je     d7afb <emuella_j2k_codestream::write_native_main_header+0x64b>
   d77d6:	66 85 db                                        	test   %bx,%bx
   d77d9:	74 75                                           	je     d7850 <emuella_j2k_codestream::write_native_main_header+0x3a0>
   d77db:	41 fe cf                                        	dec    %r15b
   d77de:	89 dd                                           	mov    %ebx,%ebp
   d77e0:	49 8b 04 24                                     	mov    (%r12),%rax
   d77e4:	4c 29 e8                                        	sub    %r13,%rax
   d77e7:	48 83 f8 02                                     	cmp    $0x2,%rax
   d77eb:	76 21                                           	jbe    d780e <emuella_j2k_codestream::write_native_main_header+0x35e>
   d77ed:	49 8b 44 24 08                                  	mov    0x8(%r12),%rax
   d77f2:	46 88 3c 28                                     	mov    %r15b,(%rax,%r13,1)
   d77f6:	66 42 c7 44 28 01 01 01                         	movw   $0x101,0x1(%rax,%r13,1)
   d77fe:	49 83 c5 03                                     	add    $0x3,%r13
   d7802:	4d 89 6c 24 10                                  	mov    %r13,0x10(%r12)
   d7807:	66 ff cd                                        	dec    %bp
   d780a:	75 d4                                           	jne    d77e0 <emuella_j2k_codestream::write_native_main_header+0x330>
   d780c:	eb 42                                           	jmp    d7850 <emuella_j2k_codestream::write_native_main_header+0x3a0>
   d780e:	ba 03 00 00 00                                  	mov    $0x3,%edx
   d7813:	b9 01 00 00 00                                  	mov    $0x1,%ecx
   d7818:	41 b8 01 00 00 00                               	mov    $0x1,%r8d
   d781e:	4c 89 e7                                        	mov    %r12,%rdi
   d7821:	4c 89 ee                                        	mov    %r13,%rsi
   d7824:	e8 c7 3e fd ff                                  	call   ab6f0 <<alloc::raw_vec::RawVecInner<_>>::reserve::do_reserve_and_handle::<alloc::alloc::Global>>
   d7829:	4d 8b 6c 24 10                                  	mov    0x10(%r12),%r13
   d782e:	eb bd                                           	jmp    d77ed <emuella_j2k_codestream::write_native_main_header+0x33d>
   d7830:	48 b8 03 00 00 00 00 00 00 80                   	movabs $0x8000000000000003,%rax
   d783a:	48 8b 4c 24 10                                  	mov    0x10(%rsp),%rcx
   d783f:	48 89 01                                        	mov    %rax,(%rcx)
   d7842:	66 85 db                                        	test   %bx,%bx
   d7845:	0f 85 d8 02 00 00                               	jne    d7b23 <emuella_j2k_codestream::write_native_main_header+0x673>
   d784b:	e9 2f 03 00 00                                  	jmp    d7b7f <emuella_j2k_codestream::write_native_main_header+0x6cf>
   d7850:	49 8b 04 24                                     	mov    (%r12),%rax
   d7854:	4c 29 e8                                        	sub    %r13,%rax
   d7857:	48 83 f8 01                                     	cmp    $0x1,%rax
   d785b:	0f 86 0e 05 00 00                               	jbe    d7d6f <emuella_j2k_codestream::write_native_main_header+0x8bf>
   d7861:	49 8b 44 24 08                                  	mov    0x8(%r12),%rax
   d7866:	66 42 c7 04 28 ff 52                            	movw   $0x52ff,(%rax,%r13,1)
   d786d:	49 83 c5 02                                     	add    $0x2,%r13
   d7871:	4d 89 6c 24 10                                  	mov    %r13,0x10(%r12)
   d7876:	49 8b 04 24                                     	mov    (%r12),%rax
   d787a:	4c 29 e8                                        	sub    %r13,%rax
   d787d:	48 83 f8 01                                     	cmp    $0x1,%rax
   d7881:	0f 86 0d 05 00 00                               	jbe    d7d94 <emuella_j2k_codestream::write_native_main_header+0x8e4>
   d7887:	49 8b 44 24 08                                  	mov    0x8(%r12),%rax
   d788c:	66 42 c7 04 28 00 0c                            	movw   $0xc00,(%rax,%r13,1)
   d7893:	49 83 c5 02                                     	add    $0x2,%r13
   d7897:	4d 89 6c 24 10                                  	mov    %r13,0x10(%r12)
   d789c:	49 8b 04 24                                     	mov    (%r12),%rax
   d78a0:	4c 29 e8                                        	sub    %r13,%rax
   d78a3:	48 83 f8 01                                     	cmp    $0x1,%rax
   d78a7:	0f 86 0c 05 00 00                               	jbe    d7db9 <emuella_j2k_codestream::write_native_main_header+0x909>
   d78ad:	49 8b 44 24 08                                  	mov    0x8(%r12),%rax
   d78b2:	66 42 c7 04 28 00 00                            	movw   $0x0,(%rax,%r13,1)
   d78b9:	49 83 c5 02                                     	add    $0x2,%r13
   d78bd:	4d 89 6c 24 10                                  	mov    %r13,0x10(%r12)
   d78c2:	49 8b 04 24                                     	mov    (%r12),%rax
   d78c6:	4c 29 e8                                        	sub    %r13,%rax
   d78c9:	48 83 f8 01                                     	cmp    $0x1,%rax
   d78cd:	0f 86 0b 05 00 00                               	jbe    d7dde <emuella_j2k_codestream::write_native_main_header+0x92e>
   d78d3:	49 8b 44 24 08                                  	mov    0x8(%r12),%rax
   d78d8:	66 42 c7 04 28 00 01                            	movw   $0x100,(%rax,%r13,1)
   d78df:	49 83 c5 02                                     	add    $0x2,%r13
   d78e3:	4d 89 6c 24 10                                  	mov    %r13,0x10(%r12)
   d78e8:	49 8b 04 24                                     	mov    (%r12),%rax
   d78ec:	4c 29 e8                                        	sub    %r13,%rax
   d78ef:	48 83 f8 05                                     	cmp    $0x5,%rax
   d78f3:	0f 86 0a 05 00 00                               	jbe    d7e03 <emuella_j2k_codestream::write_native_main_header+0x953>
   d78f9:	49 8b 44 24 08                                  	mov    0x8(%r12),%rax
   d78fe:	0f b6 4c 24 06                                  	movzbl 0x6(%rsp),%ecx
   d7903:	42 88 0c 28                                     	mov    %cl,(%rax,%r13,1)
   d7907:	66 42 c7 44 28 01 02 04                         	movw   $0x402,0x1(%rax,%r13,1)
   d790f:	42 c6 44 28 03 04                               	movb   $0x4,0x3(%rax,%r13,1)
   d7915:	0f b6 4c 24 07                                  	movzbl 0x7(%rsp),%ecx
   d791a:	42 88 4c 28 04                                  	mov    %cl,0x4(%rax,%r13,1)
   d791f:	42 c6 44 28 05 01                               	movb   $0x1,0x5(%rax,%r13,1)
   d7925:	49 83 c5 06                                     	add    $0x6,%r13
   d7929:	4d 89 6c 24 10                                  	mov    %r13,0x10(%r12)
   d792e:	49 8b 04 24                                     	mov    (%r12),%rax
   d7932:	4c 29 e8                                        	sub    %r13,%rax
   d7935:	48 83 f8 01                                     	cmp    $0x1,%rax
   d7939:	0f 86 e9 04 00 00                               	jbe    d7e28 <emuella_j2k_codestream::write_native_main_header+0x978>
   d793f:	49 8b 44 24 08                                  	mov    0x8(%r12),%rax
   d7944:	66 42 c7 04 28 ff 5c                            	movw   $0x5cff,(%rax,%r13,1)
   d794b:	49 83 c5 02                                     	add    $0x2,%r13
   d794f:	4d 89 6c 24 10                                  	mov    %r13,0x10(%r12)
   d7954:	49 8b 04 24                                     	mov    (%r12),%rax
   d7958:	4c 29 e8                                        	sub    %r13,%rax
   d795b:	48 83 f8 01                                     	cmp    $0x1,%rax
   d795f:	0f 86 e8 04 00 00                               	jbe    d7e4d <emuella_j2k_codestream::write_native_main_header+0x99d>
   d7965:	49 8b 44 24 08                                  	mov    0x8(%r12),%rax
   d796a:	66 42 c7 04 28 00 0a                            	movw   $0xa00,(%rax,%r13,1)
   d7971:	49 8d 45 02                                     	lea    0x2(%r13),%rax
   d7975:	49 89 44 24 10                                  	mov    %rax,0x10(%r12)
   d797a:	49 3b 04 24                                     	cmp    (%r12),%rax
   d797e:	75 09                                           	jne    d7989 <emuella_j2k_codestream::write_native_main_header+0x4d9>
   d7980:	4c 89 e7                                        	mov    %r12,%rdi
   d7983:	ff 15 b7 d4 19 00                               	call   *0x19d4b7(%rip)        # 274e40 <_DYNAMIC+0x290>
   d7989:	49 8b 44 24 08                                  	mov    0x8(%r12),%rax
   d798e:	42 c6 44 28 02 40                               	movb   $0x40,0x2(%rax,%r13,1)
   d7994:	49 8d 45 03                                     	lea    0x3(%r13),%rax
   d7998:	49 89 44 24 10                                  	mov    %rax,0x10(%r12)
   d799d:	48 8b 4c 24 08                                  	mov    0x8(%rsp),%rcx
   d79a2:	0f b6 29                                        	movzbl (%rcx),%ebp
   d79a5:	49 3b 04 24                                     	cmp    (%r12),%rax
   d79a9:	75 09                                           	jne    d79b4 <emuella_j2k_codestream::write_native_main_header+0x504>
   d79ab:	4c 89 e7                                        	mov    %r12,%rdi
   d79ae:	ff 15 8c d4 19 00                               	call   *0x19d48c(%rip)        # 274e40 <_DYNAMIC+0x290>
   d79b4:	40 c0 e5 03                                     	shl    $0x3,%bpl
   d79b8:	49 8b 44 24 08                                  	mov    0x8(%r12),%rax
   d79bd:	42 88 6c 28 03                                  	mov    %bpl,0x3(%rax,%r13,1)
   d79c2:	49 8d 45 04                                     	lea    0x4(%r13),%rax
   d79c6:	49 89 44 24 10                                  	mov    %rax,0x10(%r12)
   d79cb:	48 8b 4c 24 08                                  	mov    0x8(%rsp),%rcx
   d79d0:	0f b6 69 01                                     	movzbl 0x1(%rcx),%ebp
   d79d4:	49 3b 04 24                                     	cmp    (%r12),%rax
   d79d8:	75 09                                           	jne    d79e3 <emuella_j2k_codestream::write_native_main_header+0x533>
   d79da:	4c 89 e7                                        	mov    %r12,%rdi
   d79dd:	ff 15 5d d4 19 00                               	call   *0x19d45d(%rip)        # 274e40 <_DYNAMIC+0x290>
   d79e3:	40 c0 e5 03                                     	shl    $0x3,%bpl
   d79e7:	49 8b 44 24 08                                  	mov    0x8(%r12),%rax
   d79ec:	42 88 6c 28 04                                  	mov    %bpl,0x4(%rax,%r13,1)
   d79f1:	49 8d 45 05                                     	lea    0x5(%r13),%rax
   d79f5:	49 89 44 24 10                                  	mov    %rax,0x10(%r12)
   d79fa:	48 8b 4c 24 08                                  	mov    0x8(%rsp),%rcx
   d79ff:	0f b6 69 02                                     	movzbl 0x2(%rcx),%ebp
   d7a03:	49 3b 04 24                                     	cmp    (%r12),%rax
   d7a07:	75 09                                           	jne    d7a12 <emuella_j2k_codestream::write_native_main_header+0x562>
   d7a09:	4c 89 e7                                        	mov    %r12,%rdi
   d7a0c:	ff 15 2e d4 19 00                               	call   *0x19d42e(%rip)        # 274e40 <_DYNAMIC+0x290>
   d7a12:	40 c0 e5 03                                     	shl    $0x3,%bpl
   d7a16:	49 8b 44 24 08                                  	mov    0x8(%r12),%rax
   d7a1b:	42 88 6c 28 05                                  	mov    %bpl,0x5(%rax,%r13,1)
   d7a20:	49 8d 45 06                                     	lea    0x6(%r13),%rax
   d7a24:	49 89 44 24 10                                  	mov    %rax,0x10(%r12)
   d7a29:	48 8b 4c 24 08                                  	mov    0x8(%rsp),%rcx
   d7a2e:	0f b6 69 03                                     	movzbl 0x3(%rcx),%ebp
   d7a32:	49 3b 04 24                                     	cmp    (%r12),%rax
   d7a36:	75 09                                           	jne    d7a41 <emuella_j2k_codestream::write_native_main_header+0x591>
   d7a38:	4c 89 e7                                        	mov    %r12,%rdi
   d7a3b:	ff 15 ff d3 19 00                               	call   *0x19d3ff(%rip)        # 274e40 <_DYNAMIC+0x290>
   d7a41:	40 c0 e5 03                                     	shl    $0x3,%bpl
   d7a45:	49 8b 44 24 08                                  	mov    0x8(%r12),%rax
   d7a4a:	42 88 6c 28 06                                  	mov    %bpl,0x6(%rax,%r13,1)
   d7a4f:	49 8d 45 07                                     	lea    0x7(%r13),%rax
   d7a53:	49 89 44 24 10                                  	mov    %rax,0x10(%r12)
   d7a58:	48 8b 4c 24 08                                  	mov    0x8(%rsp),%rcx
   d7a5d:	0f b6 69 04                                     	movzbl 0x4(%rcx),%ebp
   d7a61:	49 3b 04 24                                     	cmp    (%r12),%rax
   d7a65:	75 09                                           	jne    d7a70 <emuella_j2k_codestream::write_native_main_header+0x5c0>
   d7a67:	4c 89 e7                                        	mov    %r12,%rdi
   d7a6a:	ff 15 d0 d3 19 00                               	call   *0x19d3d0(%rip)        # 274e40 <_DYNAMIC+0x290>
   d7a70:	40 c0 e5 03                                     	shl    $0x3,%bpl
   d7a74:	49 8b 44 24 08                                  	mov    0x8(%r12),%rax
   d7a79:	42 88 6c 28 07                                  	mov    %bpl,0x7(%rax,%r13,1)
   d7a7e:	49 8d 45 08                                     	lea    0x8(%r13),%rax
   d7a82:	49 89 44 24 10                                  	mov    %rax,0x10(%r12)
   d7a87:	48 8b 4c 24 08                                  	mov    0x8(%rsp),%rcx
   d7a8c:	0f b6 69 05                                     	movzbl 0x5(%rcx),%ebp
   d7a90:	49 3b 04 24                                     	cmp    (%r12),%rax
   d7a94:	75 09                                           	jne    d7a9f <emuella_j2k_codestream::write_native_main_header+0x5ef>
   d7a96:	4c 89 e7                                        	mov    %r12,%rdi
   d7a99:	ff 15 a1 d3 19 00                               	call   *0x19d3a1(%rip)        # 274e40 <_DYNAMIC+0x290>
   d7a9f:	40 c0 e5 03                                     	shl    $0x3,%bpl
   d7aa3:	49 8b 44 24 08                                  	mov    0x8(%r12),%rax
   d7aa8:	42 88 6c 28 08                                  	mov    %bpl,0x8(%rax,%r13,1)
   d7aad:	49 8d 45 09                                     	lea    0x9(%r13),%rax
   d7ab1:	49 89 44 24 10                                  	mov    %rax,0x10(%r12)
   d7ab6:	48 8b 4c 24 08                                  	mov    0x8(%rsp),%rcx
   d7abb:	0f b6 69 06                                     	movzbl 0x6(%rcx),%ebp
   d7abf:	49 3b 04 24                                     	cmp    (%r12),%rax
   d7ac3:	75 09                                           	jne    d7ace <emuella_j2k_codestream::write_native_main_header+0x61e>
   d7ac5:	4c 89 e7                                        	mov    %r12,%rdi
   d7ac8:	ff 15 72 d3 19 00                               	call   *0x19d372(%rip)        # 274e40 <_DYNAMIC+0x290>
   d7ace:	40 c0 e5 03                                     	shl    $0x3,%bpl
   d7ad2:	49 8b 44 24 08                                  	mov    0x8(%r12),%rax
   d7ad7:	42 88 6c 28 09                                  	mov    %bpl,0x9(%rax,%r13,1)
   d7adc:	49 83 c5 0a                                     	add    $0xa,%r13
   d7ae0:	4d 89 6c 24 10                                  	mov    %r13,0x10(%r12)
   d7ae5:	48 8b 44 24 10                                  	mov    0x10(%rsp),%rax
   d7aea:	48 c7 00 ff ff ff ff                            	movq   $0xffffffffffffffff,(%rax)
   d7af1:	66 85 db                                        	test   %bx,%bx
   d7af4:	75 2d                                           	jne    d7b23 <emuella_j2k_codestream::write_native_main_header+0x673>
   d7af6:	e9 84 00 00 00                                  	jmp    d7b7f <emuella_j2k_codestream::write_native_main_header+0x6cf>
   d7afb:	48 b8 03 00 00 00 00 00 00 80                   	movabs $0x8000000000000003,%rax
   d7b05:	48 8b 4c 24 10                                  	mov    0x10(%rsp),%rcx
   d7b0a:	48 89 01                                        	mov    %rax,(%rcx)
   d7b0d:	c6 41 0f 00                                     	movb   $0x0,0xf(%rcx)
   d7b11:	66 c7 41 0d 00 00                               	movw   $0x0,0xd(%rcx)
   d7b17:	c7 41 09 00 00 00 00                            	movl   $0x0,0x9(%rcx)
   d7b1e:	66 85 db                                        	test   %bx,%bx
   d7b21:	74 5c                                           	je     d7b7f <emuella_j2k_codestream::write_native_main_header+0x6cf>
   d7b23:	4c 89 f7                                        	mov    %r14,%rdi
   d7b26:	48 83 c4 28                                     	add    $0x28,%rsp
   d7b2a:	5b                                              	pop    %rbx
   d7b2b:	41 5c                                           	pop    %r12
   d7b2d:	41 5d                                           	pop    %r13
   d7b2f:	41 5e                                           	pop    %r14
   d7b31:	41 5f                                           	pop    %r15
   d7b33:	5d                                              	pop    %rbp
   d7b34:	ff 25 8e d2 19 00                               	jmp    *0x19d28e(%rip)        # 274dc8 <free@GLIBC_2.2.5>
   d7b3a:	48 b8 03 00 00 00 00 00 00 80                   	movabs $0x8000000000000003,%rax
   d7b44:	48 8b 4c 24 10                                  	mov    0x10(%rsp),%rcx
   d7b49:	48 89 01                                        	mov    %rax,(%rcx)
   d7b4c:	66 c7 41 0e 00 00                               	movw   $0x0,0xe(%rcx)
   d7b52:	c7 41 0a 00 00 00 00                            	movl   $0x0,0xa(%rcx)
   d7b59:	eb c8                                           	jmp    d7b23 <emuella_j2k_codestream::write_native_main_header+0x673>
   d7b5b:	48 b8 03 00 00 00 00 00 00 80                   	movabs $0x8000000000000003,%rax
   d7b65:	48 8b 4c 24 10                                  	mov    0x10(%rsp),%rcx
   d7b6a:	48 89 01                                        	mov    %rax,(%rcx)
   d7b6d:	66 c7 41 0e 00 00                               	movw   $0x0,0xe(%rcx)
   d7b73:	c7 41 0a 00 00 00 00                            	movl   $0x0,0xa(%rcx)
   d7b7a:	66 85 db                                        	test   %bx,%bx
   d7b7d:	75 a4                                           	jne    d7b23 <emuella_j2k_codestream::write_native_main_header+0x673>
   d7b7f:	48 83 c4 28                                     	add    $0x28,%rsp
   d7b83:	5b                                              	pop    %rbx
   d7b84:	41 5c                                           	pop    %r12
   d7b86:	41 5d                                           	pop    %r13
   d7b88:	41 5e                                           	pop    %r14
   d7b8a:	41 5f                                           	pop    %r15
   d7b8c:	5d                                              	pop    %rbp
   d7b8d:	c3                                              	ret
   d7b8e:	ba 02 00 00 00                                  	mov    $0x2,%edx
   d7b93:	b9 01 00 00 00                                  	mov    $0x1,%ecx
   d7b98:	41 b8 01 00 00 00                               	mov    $0x1,%r8d
   d7b9e:	4c 89 e7                                        	mov    %r12,%rdi
   d7ba1:	4c 89 ee                                        	mov    %r13,%rsi
   d7ba4:	e8 47 3b fd ff                                  	call   ab6f0 <<alloc::raw_vec::RawVecInner<_>>::reserve::do_reserve_and_handle::<alloc::alloc::Global>>
   d7ba9:	4d 8b 6c 24 10                                  	mov    0x10(%r12),%r13
   d7bae:	e9 24 fa ff ff                                  	jmp    d75d7 <emuella_j2k_codestream::write_native_main_header+0x127>
   d7bb3:	ba 02 00 00 00                                  	mov    $0x2,%edx
   d7bb8:	b9 01 00 00 00                                  	mov    $0x1,%ecx
   d7bbd:	41 b8 01 00 00 00                               	mov    $0x1,%r8d
   d7bc3:	4c 89 e7                                        	mov    %r12,%rdi
   d7bc6:	4c 89 ee                                        	mov    %r13,%rsi
   d7bc9:	e8 22 3b fd ff                                  	call   ab6f0 <<alloc::raw_vec::RawVecInner<_>>::reserve::do_reserve_and_handle::<alloc::alloc::Global>>
   d7bce:	4d 8b 6c 24 10                                  	mov    0x10(%r12),%r13
   d7bd3:	e9 25 fa ff ff                                  	jmp    d75fd <emuella_j2k_codestream::write_native_main_header+0x14d>
   d7bd8:	ba 02 00 00 00                                  	mov    $0x2,%edx
   d7bdd:	b9 01 00 00 00                                  	mov    $0x1,%ecx
   d7be2:	41 b8 01 00 00 00                               	mov    $0x1,%r8d
   d7be8:	4c 89 e7                                        	mov    %r12,%rdi
   d7beb:	4c 89 ee                                        	mov    %r13,%rsi
   d7bee:	e8 fd 3a fd ff                                  	call   ab6f0 <<alloc::raw_vec::RawVecInner<_>>::reserve::do_reserve_and_handle::<alloc::alloc::Global>>
   d7bf3:	4d 8b 6c 24 10                                  	mov    0x10(%r12),%r13
   d7bf8:	e9 26 fa ff ff                                  	jmp    d7623 <emuella_j2k_codestream::write_native_main_header+0x173>
   d7bfd:	ba 02 00 00 00                                  	mov    $0x2,%edx
   d7c02:	b9 01 00 00 00                                  	mov    $0x1,%ecx
   d7c07:	41 b8 01 00 00 00                               	mov    $0x1,%r8d
   d7c0d:	4c 89 e7                                        	mov    %r12,%rdi
   d7c10:	4c 89 ee                                        	mov    %r13,%rsi
   d7c13:	e8 d8 3a fd ff                                  	call   ab6f0 <<alloc::raw_vec::RawVecInner<_>>::reserve::do_reserve_and_handle::<alloc::alloc::Global>>
   d7c18:	4d 8b 6c 24 10                                  	mov    0x10(%r12),%r13
   d7c1d:	e9 2c fa ff ff                                  	jmp    d764e <emuella_j2k_codestream::write_native_main_header+0x19e>
   d7c22:	ba 04 00 00 00                                  	mov    $0x4,%edx
   d7c27:	b9 01 00 00 00                                  	mov    $0x1,%ecx
   d7c2c:	41 b8 01 00 00 00                               	mov    $0x1,%r8d
   d7c32:	4c 89 e7                                        	mov    %r12,%rdi
   d7c35:	4c 89 ee                                        	mov    %r13,%rsi
   d7c38:	e8 b3 3a fd ff                                  	call   ab6f0 <<alloc::raw_vec::RawVecInner<_>>::reserve::do_reserve_and_handle::<alloc::alloc::Global>>
   d7c3d:	4d 8b 6c 24 10                                  	mov    0x10(%r12),%r13
   d7c42:	e9 2d fa ff ff                                  	jmp    d7674 <emuella_j2k_codestream::write_native_main_header+0x1c4>
   d7c47:	ba 04 00 00 00                                  	mov    $0x4,%edx
   d7c4c:	b9 01 00 00 00                                  	mov    $0x1,%ecx
   d7c51:	41 b8 01 00 00 00                               	mov    $0x1,%r8d
   d7c57:	4c 89 e7                                        	mov    %r12,%rdi
   d7c5a:	4c 89 ee                                        	mov    %r13,%rsi
   d7c5d:	e8 8e 3a fd ff                                  	call   ab6f0 <<alloc::raw_vec::RawVecInner<_>>::reserve::do_reserve_and_handle::<alloc::alloc::Global>>
   d7c62:	4d 8b 6c 24 10                                  	mov    0x10(%r12),%r13
   d7c67:	e9 31 fa ff ff                                  	jmp    d769d <emuella_j2k_codestream::write_native_main_header+0x1ed>
   d7c6c:	ba 04 00 00 00                                  	mov    $0x4,%edx
   d7c71:	b9 01 00 00 00                                  	mov    $0x1,%ecx
   d7c76:	41 b8 01 00 00 00                               	mov    $0x1,%r8d
   d7c7c:	4c 89 e7                                        	mov    %r12,%rdi
   d7c7f:	4c 89 ee                                        	mov    %r13,%rsi
   d7c82:	e8 69 3a fd ff                                  	call   ab6f0 <<alloc::raw_vec::RawVecInner<_>>::reserve::do_reserve_and_handle::<alloc::alloc::Global>>
   d7c87:	4d 8b 6c 24 10                                  	mov    0x10(%r12),%r13
   d7c8c:	e9 35 fa ff ff                                  	jmp    d76c6 <emuella_j2k_codestream::write_native_main_header+0x216>
   d7c91:	ba 04 00 00 00                                  	mov    $0x4,%edx
   d7c96:	b9 01 00 00 00                                  	mov    $0x1,%ecx
   d7c9b:	41 b8 01 00 00 00                               	mov    $0x1,%r8d
   d7ca1:	4c 89 e7                                        	mov    %r12,%rdi
   d7ca4:	4c 89 ee                                        	mov    %r13,%rsi
   d7ca7:	e8 44 3a fd ff                                  	call   ab6f0 <<alloc::raw_vec::RawVecInner<_>>::reserve::do_reserve_and_handle::<alloc::alloc::Global>>
   d7cac:	4d 8b 6c 24 10                                  	mov    0x10(%r12),%r13
   d7cb1:	e9 37 fa ff ff                                  	jmp    d76ed <emuella_j2k_codestream::write_native_main_header+0x23d>
   d7cb6:	ba 04 00 00 00                                  	mov    $0x4,%edx
   d7cbb:	b9 01 00 00 00                                  	mov    $0x1,%ecx
   d7cc0:	41 b8 01 00 00 00                               	mov    $0x1,%r8d
   d7cc6:	4c 89 e7                                        	mov    %r12,%rdi
   d7cc9:	4c 89 ee                                        	mov    %r13,%rsi
   d7ccc:	e8 1f 3a fd ff                                  	call   ab6f0 <<alloc::raw_vec::RawVecInner<_>>::reserve::do_reserve_and_handle::<alloc::alloc::Global>>
   d7cd1:	4d 8b 6c 24 10                                  	mov    0x10(%r12),%r13
   d7cd6:	e9 39 fa ff ff                                  	jmp    d7714 <emuella_j2k_codestream::write_native_main_header+0x264>
   d7cdb:	ba 04 00 00 00                                  	mov    $0x4,%edx
   d7ce0:	b9 01 00 00 00                                  	mov    $0x1,%ecx
   d7ce5:	41 b8 01 00 00 00                               	mov    $0x1,%r8d
   d7ceb:	4c 89 e7                                        	mov    %r12,%rdi
   d7cee:	4c 89 ee                                        	mov    %r13,%rsi
   d7cf1:	e8 fa 39 fd ff                                  	call   ab6f0 <<alloc::raw_vec::RawVecInner<_>>::reserve::do_reserve_and_handle::<alloc::alloc::Global>>
   d7cf6:	4d 8b 6c 24 10                                  	mov    0x10(%r12),%r13
   d7cfb:	e9 3d fa ff ff                                  	jmp    d773d <emuella_j2k_codestream::write_native_main_header+0x28d>
   d7d00:	ba 04 00 00 00                                  	mov    $0x4,%edx
   d7d05:	b9 01 00 00 00                                  	mov    $0x1,%ecx
   d7d0a:	41 b8 01 00 00 00                               	mov    $0x1,%r8d
   d7d10:	4c 89 e7                                        	mov    %r12,%rdi
   d7d13:	4c 89 ee                                        	mov    %r13,%rsi
   d7d16:	e8 d5 39 fd ff                                  	call   ab6f0 <<alloc::raw_vec::RawVecInner<_>>::reserve::do_reserve_and_handle::<alloc::alloc::Global>>
   d7d1b:	4d 8b 6c 24 10                                  	mov    0x10(%r12),%r13
   d7d20:	e9 41 fa ff ff                                  	jmp    d7766 <emuella_j2k_codestream::write_native_main_header+0x2b6>
   d7d25:	ba 04 00 00 00                                  	mov    $0x4,%edx
   d7d2a:	b9 01 00 00 00                                  	mov    $0x1,%ecx
   d7d2f:	41 b8 01 00 00 00                               	mov    $0x1,%r8d
   d7d35:	4c 89 e7                                        	mov    %r12,%rdi
   d7d38:	4c 89 ee                                        	mov    %r13,%rsi
   d7d3b:	e8 b0 39 fd ff                                  	call   ab6f0 <<alloc::raw_vec::RawVecInner<_>>::reserve::do_reserve_and_handle::<alloc::alloc::Global>>
   d7d40:	4d 8b 6c 24 10                                  	mov    0x10(%r12),%r13
   d7d45:	e9 43 fa ff ff                                  	jmp    d778d <emuella_j2k_codestream::write_native_main_header+0x2dd>
   d7d4a:	ba 02 00 00 00                                  	mov    $0x2,%edx
   d7d4f:	b9 01 00 00 00                                  	mov    $0x1,%ecx
   d7d54:	41 b8 01 00 00 00                               	mov    $0x1,%r8d
   d7d5a:	4c 89 e7                                        	mov    %r12,%rdi
   d7d5d:	4c 89 ee                                        	mov    %r13,%rsi
   d7d60:	e8 8b 39 fd ff                                  	call   ab6f0 <<alloc::raw_vec::RawVecInner<_>>::reserve::do_reserve_and_handle::<alloc::alloc::Global>>
   d7d65:	4d 8b 6c 24 10                                  	mov    0x10(%r12),%r13
   d7d6a:	e9 45 fa ff ff                                  	jmp    d77b4 <emuella_j2k_codestream::write_native_main_header+0x304>
   d7d6f:	ba 02 00 00 00                                  	mov    $0x2,%edx
   d7d74:	b9 01 00 00 00                                  	mov    $0x1,%ecx
   d7d79:	41 b8 01 00 00 00                               	mov    $0x1,%r8d
   d7d7f:	4c 89 e7                                        	mov    %r12,%rdi
   d7d82:	4c 89 ee                                        	mov    %r13,%rsi
   d7d85:	e8 66 39 fd ff                                  	call   ab6f0 <<alloc::raw_vec::RawVecInner<_>>::reserve::do_reserve_and_handle::<alloc::alloc::Global>>
   d7d8a:	4d 8b 6c 24 10                                  	mov    0x10(%r12),%r13
   d7d8f:	e9 cd fa ff ff                                  	jmp    d7861 <emuella_j2k_codestream::write_native_main_header+0x3b1>
   d7d94:	ba 02 00 00 00                                  	mov    $0x2,%edx
   d7d99:	b9 01 00 00 00                                  	mov    $0x1,%ecx
   d7d9e:	41 b8 01 00 00 00                               	mov    $0x1,%r8d
   d7da4:	4c 89 e7                                        	mov    %r12,%rdi
   d7da7:	4c 89 ee                                        	mov    %r13,%rsi
   d7daa:	e8 41 39 fd ff                                  	call   ab6f0 <<alloc::raw_vec::RawVecInner<_>>::reserve::do_reserve_and_handle::<alloc::alloc::Global>>
   d7daf:	4d 8b 6c 24 10                                  	mov    0x10(%r12),%r13
   d7db4:	e9 ce fa ff ff                                  	jmp    d7887 <emuella_j2k_codestream::write_native_main_header+0x3d7>
   d7db9:	ba 02 00 00 00                                  	mov    $0x2,%edx
   d7dbe:	b9 01 00 00 00                                  	mov    $0x1,%ecx
   d7dc3:	41 b8 01 00 00 00                               	mov    $0x1,%r8d
   d7dc9:	4c 89 e7                                        	mov    %r12,%rdi
   d7dcc:	4c 89 ee                                        	mov    %r13,%rsi
   d7dcf:	e8 1c 39 fd ff                                  	call   ab6f0 <<alloc::raw_vec::RawVecInner<_>>::reserve::do_reserve_and_handle::<alloc::alloc::Global>>
   d7dd4:	4d 8b 6c 24 10                                  	mov    0x10(%r12),%r13
   d7dd9:	e9 cf fa ff ff                                  	jmp    d78ad <emuella_j2k_codestream::write_native_main_header+0x3fd>
   d7dde:	ba 02 00 00 00                                  	mov    $0x2,%edx
   d7de3:	b9 01 00 00 00                                  	mov    $0x1,%ecx
   d7de8:	41 b8 01 00 00 00                               	mov    $0x1,%r8d
   d7dee:	4c 89 e7                                        	mov    %r12,%rdi
   d7df1:	4c 89 ee                                        	mov    %r13,%rsi
   d7df4:	e8 f7 38 fd ff                                  	call   ab6f0 <<alloc::raw_vec::RawVecInner<_>>::reserve::do_reserve_and_handle::<alloc::alloc::Global>>
   d7df9:	4d 8b 6c 24 10                                  	mov    0x10(%r12),%r13
   d7dfe:	e9 d0 fa ff ff                                  	jmp    d78d3 <emuella_j2k_codestream::write_native_main_header+0x423>
   d7e03:	ba 06 00 00 00                                  	mov    $0x6,%edx
   d7e08:	b9 01 00 00 00                                  	mov    $0x1,%ecx
   d7e0d:	41 b8 01 00 00 00                               	mov    $0x1,%r8d
   d7e13:	4c 89 e7                                        	mov    %r12,%rdi
   d7e16:	4c 89 ee                                        	mov    %r13,%rsi
   d7e19:	e8 d2 38 fd ff                                  	call   ab6f0 <<alloc::raw_vec::RawVecInner<_>>::reserve::do_reserve_and_handle::<alloc::alloc::Global>>
   d7e1e:	4d 8b 6c 24 10                                  	mov    0x10(%r12),%r13
   d7e23:	e9 d1 fa ff ff                                  	jmp    d78f9 <emuella_j2k_codestream::write_native_main_header+0x449>
   d7e28:	ba 02 00 00 00                                  	mov    $0x2,%edx
   d7e2d:	b9 01 00 00 00                                  	mov    $0x1,%ecx
   d7e32:	41 b8 01 00 00 00                               	mov    $0x1,%r8d
   d7e38:	4c 89 e7                                        	mov    %r12,%rdi
   d7e3b:	4c 89 ee                                        	mov    %r13,%rsi
   d7e3e:	e8 ad 38 fd ff                                  	call   ab6f0 <<alloc::raw_vec::RawVecInner<_>>::reserve::do_reserve_and_handle::<alloc::alloc::Global>>
   d7e43:	4d 8b 6c 24 10                                  	mov    0x10(%r12),%r13
   d7e48:	e9 f2 fa ff ff                                  	jmp    d793f <emuella_j2k_codestream::write_native_main_header+0x48f>
   d7e4d:	ba 02 00 00 00                                  	mov    $0x2,%edx
   d7e52:	b9 01 00 00 00                                  	mov    $0x1,%ecx
   d7e57:	41 b8 01 00 00 00                               	mov    $0x1,%r8d
   d7e5d:	4c 89 e7                                        	mov    %r12,%rdi
   d7e60:	4c 89 ee                                        	mov    %r13,%rsi
   d7e63:	e8 88 38 fd ff                                  	call   ab6f0 <<alloc::raw_vec::RawVecInner<_>>::reserve::do_reserve_and_handle::<alloc::alloc::Global>>
   d7e68:	4d 8b 6c 24 10                                  	mov    0x10(%r12),%r13
   d7e6d:	e9 f3 fa ff ff                                  	jmp    d7965 <emuella_j2k_codestream::write_native_main_header+0x4b5>
   d7e72:	bf 01 00 00 00                                  	mov    $0x1,%edi
   d7e77:	4c 89 ee                                        	mov    %r13,%rsi
   d7e7a:	ff 15 80 cf 19 00                               	call   *0x19cf80(%rip)        # 274e00 <_DYNAMIC+0x250>
   d7e80:	eb 00                                           	jmp    d7e82 <emuella_j2k_codestream::write_native_main_header+0x9d2>
   d7e82:	49 89 c7                                        	mov    %rax,%r15
   d7e85:	66 85 db                                        	test   %bx,%bx
   d7e88:	74 09                                           	je     d7e93 <emuella_j2k_codestream::write_native_main_header+0x9e3>
   d7e8a:	4c 89 f7                                        	mov    %r14,%rdi
   d7e8d:	ff 15 35 cf 19 00                               	call   *0x19cf35(%rip)        # 274dc8 <free@GLIBC_2.2.5>
   d7e93:	4c 89 ff                                        	mov    %r15,%rdi
   d7e96:	e8 05 43 19 00                                  	call   26c1a0 <_Unwind_Resume@plt>
