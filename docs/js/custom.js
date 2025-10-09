// Custom JavaScript for imgrs documentation

// Copy to clipboard function
function copyToClipboard(text) {
    if (navigator.clipboard) {
        navigator.clipboard.writeText(text).then(() => {
            // Show success feedback
            const btn = event.target.closest('button');
            const originalHTML = btn.innerHTML;
            btn.innerHTML = '<i class="bi bi-check-lg"></i> Copied!';
            btn.classList.add('btn-success');
            btn.classList.remove('btn-warning');
            
            setTimeout(() => {
                btn.innerHTML = originalHTML;
                btn.classList.add('btn-warning');
                btn.classList.remove('btn-success');
            }, 2000);
        }).catch(err => {
            console.error('Failed to copy:', err);
            alert('Copy failed. Please copy manually: ' + text);
        });
    } else {
        // Fallback for older browsers
        const textArea = document.createElement('textarea');
        textArea.value = text;
        textArea.style.position = 'fixed';
        textArea.style.left = '-999999px';
        document.body.appendChild(textArea);
        textArea.select();
        try {
            document.execCommand('copy');
            alert('Copied to clipboard!');
        } catch (err) {
            alert('Copy failed. Please copy manually: ' + text);
        }
        document.body.removeChild(textArea);
    }
}

// Smooth scroll for anchor links
document.querySelectorAll('a[href^="#"]').forEach(anchor => {
    anchor.addEventListener('click', function (e) {
        e.preventDefault();
        const target = document.querySelector(this.getAttribute('href'));
        if (target) {
            target.scrollIntoView({
                behavior: 'smooth',
                block: 'start'
            });
        }
    });
});

// Add copy buttons to code blocks
document.addEventListener('DOMContentLoaded', () => {
    document.querySelectorAll('pre code').forEach((block) => {
        // Create copy button
        const button = document.createElement('button');
        button.className = 'btn btn-sm btn-outline-secondary position-absolute top-0 end-0 m-2';
        button.innerHTML = '<i class="bi bi-clipboard"></i>';
        button.title = 'Copy code';
        
        // Make parent relative for absolute positioning
        const pre = block.parentElement;
        pre.style.position = 'relative';
        
        // Add click handler
        button.addEventListener('click', () => {
            const code = block.textContent;
            copyToClipboard(code);
            button.innerHTML = '<i class="bi bi-check-lg"></i>';
            setTimeout(() => {
                button.innerHTML = '<i class="bi bi-clipboard"></i>';
            }, 2000);
        });
        
        pre.appendChild(button);
    });
});

// Animate counters on scroll
const animateCounter = (element, target) => {
    let current = 0;
    const increment = target / 100;
    const timer = setInterval(() => {
        current += increment;
        if (current >= target) {
            element.textContent = target + 'x';
            clearInterval(timer);
        } else {
            element.textContent = Math.floor(current) + 'x';
        }
    }, 20);
};

// Intersection Observer for animations
const observer = new IntersectionObserver((entries) => {
    entries.forEach(entry => {
        if (entry.isIntersecting) {
            entry.target.classList.add('animate__fadeIn');
        }
    });
}, {
    threshold: 0.1
});

// Observe all cards for animation
document.addEventListener('DOMContentLoaded', () => {
    document.querySelectorAll('.card').forEach(card => {
        observer.observe(card);
    });
});

// Add active class to navbar on scroll
window.addEventListener('scroll', () => {
    const sections = document.querySelectorAll('section[id]');
    const scrollY = window.pageYOffset;

    sections.forEach(section => {
        const sectionHeight = section.offsetHeight;
        const sectionTop = section.offsetTop - 100;
        const sectionId = section.getAttribute('id');
        
        if (scrollY > sectionTop && scrollY <= sectionTop + sectionHeight) {
            document.querySelectorAll('.nav-link').forEach(link => {
                link.classList.remove('active');
                if (link.getAttribute('href') === `#${sectionId}`) {
                    link.classList.add('active');
                }
            });
        }
    });
});

// Performance comparison chart (simple)
function createPerformanceChart() {
    const benchmarkSection = document.querySelector('#benchmark');
    if (!benchmarkSection) return;
    
    // Add visual bars for performance comparison
    const performanceData = [
        { name: 'Open', imgrs: 0.00, pillow: 0.49, winner: 'imgrs' },
        { name: 'Save', imgrs: 15.75, pillow: 134.11, winner: 'imgrs' },
        { name: 'Resize', imgrs: 10.59, pillow: 16.33, winner: 'imgrs' },
    ];
    
    // Could add Chart.js here for visual charts
    // For now, keeping it minimal with Bootstrap
}

// Initialize on load
document.addEventListener('DOMContentLoaded', createPerformanceChart);

// Add tooltip initialization
const tooltipTriggerList = document.querySelectorAll('[data-bs-toggle="tooltip"]');
const tooltipList = [...tooltipTriggerList].map(tooltipTriggerEl => new bootstrap.Tooltip(tooltipTriggerEl));

console.log('📸 imgrs documentation loaded - v0.1.0');

