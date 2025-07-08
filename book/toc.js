// Populate the sidebar
//
// This is a script, and not included directly in the page, to control the total size of the book.
// The TOC contains an entry for each page, so if each page includes a copy of the TOC,
// the total size of the page becomes O(n**2).
class MDBookSidebarScrollbox extends HTMLElement {
    constructor() {
        super();
    }
    connectedCallback() {
        this.innerHTML = '<ol class="chapter"><li class="chapter-item expanded "><a href="introduction.html">Introduction</a></li><li class="chapter-item expanded "><a href="how-to-use.html">How to Use This Book</a></li><li class="chapter-item expanded affix "><li class="spacer"></li><li class="chapter-item expanded affix "><li class="part-title">Part 1 — Why Rust Meets Cryptography</li><li class="chapter-item expanded "><a href="01-foundations/01-01-cryptography.html">1.1. Cryptography is a Systems Problem</a></li><li class="chapter-item expanded "><a href="01-foundations/01-02-rust-offers.html">1.2. Safety, Performance, Predictability</a></li><li class="chapter-item expanded "><a href="01-foundations/01-03-cost-unsafety.html">1.3. Cost of Unsafety: Famous Failures</a></li><li class="chapter-item expanded "><a href="01-foundations/01-04-first-code.html">1.4. First Code: A Naive XOR Encryptor</a></li><li class="chapter-item expanded "><a href="01-foundations/01-05-tooling-up.html">1.5. Tooling Up</a></li><li class="chapter-item expanded affix "><li class="part-title">Part 2 — Core Cryptographic Primitives</li><li class="chapter-item expanded "><a href="02-core-primitives/02-01-symmetric-ciphers.html">2.1. Symmetric Ciphers</a></li><li class="chapter-item expanded "><a href="02-core-primitives/02-02-crypto-hashes.html">2.2. Cryptographic Hashes</a></li><li class="chapter-item expanded "><a href="02-core-primitives/02-03-mac-aead.html">2.3. MACs &amp; AEAD</a></li><li class="chapter-item expanded "><a href="02-core-primitives/02-04-public-key.html">2.4. Public-Key Cryptography</a></li><li class="chapter-item expanded "><a href="02-core-primitives/02-05-digital-signature.html">2.5. Digital Signatures</a></li><li class="chapter-item expanded "><a href="02-core-primitives/02-06-key-derivation.html">2.6. Key Derivation Functions</a></li><li class="chapter-item expanded "><a href="02-core-primitives/02-07-randomness-entropy.html">2.7. Randomness &amp; Entropy</a></li><li class="chapter-item expanded "><a href="02-core-primitives/02-08-merkle-tree.html">2.8. Merkle Trees &amp; Hash Chains</a></li><li class="chapter-item expanded "><a href="02-core-primitives/02-09-zkp.html">2.9. Zero-Knowledge Proofs (ZKPs)</a></li><li class="chapter-item expanded "><a href="02-core-primitives/02-10-post-quantum.html">2.10. Post-Quantum Crypto</a></li></ol>';
        // Set the current, active page, and reveal it if it's hidden
        let current_page = document.location.href.toString().split("#")[0].split("?")[0];
        if (current_page.endsWith("/")) {
            current_page += "index.html";
        }
        var links = Array.prototype.slice.call(this.querySelectorAll("a"));
        var l = links.length;
        for (var i = 0; i < l; ++i) {
            var link = links[i];
            var href = link.getAttribute("href");
            if (href && !href.startsWith("#") && !/^(?:[a-z+]+:)?\/\//.test(href)) {
                link.href = path_to_root + href;
            }
            // The "index" page is supposed to alias the first chapter in the book.
            if (link.href === current_page || (i === 0 && path_to_root === "" && current_page.endsWith("/index.html"))) {
                link.classList.add("active");
                var parent = link.parentElement;
                if (parent && parent.classList.contains("chapter-item")) {
                    parent.classList.add("expanded");
                }
                while (parent) {
                    if (parent.tagName === "LI" && parent.previousElementSibling) {
                        if (parent.previousElementSibling.classList.contains("chapter-item")) {
                            parent.previousElementSibling.classList.add("expanded");
                        }
                    }
                    parent = parent.parentElement;
                }
            }
        }
        // Track and set sidebar scroll position
        this.addEventListener('click', function(e) {
            if (e.target.tagName === 'A') {
                sessionStorage.setItem('sidebar-scroll', this.scrollTop);
            }
        }, { passive: true });
        var sidebarScrollTop = sessionStorage.getItem('sidebar-scroll');
        sessionStorage.removeItem('sidebar-scroll');
        if (sidebarScrollTop) {
            // preserve sidebar scroll position when navigating via links within sidebar
            this.scrollTop = sidebarScrollTop;
        } else {
            // scroll sidebar to current active section when navigating via "next/previous chapter" buttons
            var activeSection = document.querySelector('#sidebar .active');
            if (activeSection) {
                activeSection.scrollIntoView({ block: 'center' });
            }
        }
        // Toggle buttons
        var sidebarAnchorToggles = document.querySelectorAll('#sidebar a.toggle');
        function toggleSection(ev) {
            ev.currentTarget.parentElement.classList.toggle('expanded');
        }
        Array.from(sidebarAnchorToggles).forEach(function (el) {
            el.addEventListener('click', toggleSection);
        });
    }
}
window.customElements.define("mdbook-sidebar-scrollbox", MDBookSidebarScrollbox);
