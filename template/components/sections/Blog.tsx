import { Container } from "@/components/ui/Container";
import { SectionHeading } from "@/components/ui/SectionHeading";
import { ButtonLink } from "@/components/ui/ButtonLink";
import { portfolio } from "@/lib/portfolio";

export function Blog() {
  const blog = portfolio.blog;
  if (!blog.enabled) return null;
  const posts = blog.posts ?? [];
  if (!posts.length && !blog.url) return null;

  return (
    <section id="blog" className="section-pad" aria-labelledby="blog-heading">
      <Container>
        <div className="flex flex-wrap items-end justify-between gap-4">
          <SectionHeading
            id="blog-heading"
            title={blog.title ?? "Blog"}
            subtitle={blog.subtitle}
            className="mb-0"
          />
          {blog.url ? (
            <ButtonLink href={blog.url} variant="secondary">
              All posts
            </ButtonLink>
          ) : null}
        </div>
        {posts.length ? (
          <ul className="mt-10 space-y-4">
            {posts.map((post) => (
              <li key={post.url} className="card-surface p-5 sm:p-6">
                <a
                  href={post.url}
                  className="text-lg font-semibold text-primary underline-offset-4 hover:underline"
                  target="_blank"
                  rel="noopener noreferrer"
                >
                  {post.title}
                </a>
                {post.date ? (
                  <p className="mt-1 text-xs text-muted">{post.date}</p>
                ) : null}
                {post.summary ? (
                  <p className="mt-2 text-sm text-muted">{post.summary}</p>
                ) : null}
              </li>
            ))}
          </ul>
        ) : null}
      </Container>
    </section>
  );
}
